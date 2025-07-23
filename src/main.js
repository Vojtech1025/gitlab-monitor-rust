const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const { appWindow } = window.__TAURI__.window;

let refreshButtonEl;
let retryButtonEl;
let releasesListEl;
let loadingEl;
let errorEl;
let errorMessageEl;
let releasesContainerEl;
let emptyStateEl;
let lastUpdatedEl;

// Application state
let releases = [];
let isLoading = false;
let newReleaseIds = new Set(); // Track which releases are new

// Format date to relative time
function formatRelativeTime(date) {
  const now = new Date();
  const diff = now - date;
  const seconds = Math.floor(diff / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  if (days > 0) return `${days} day${days === 1 ? '' : 's'} ago`;
  if (hours > 0) return `${hours} hour${hours === 1 ? '' : 's'} ago`;
  if (minutes > 0) return `${minutes} minute${minutes === 1 ? '' : 's'} ago`;
  return 'Just now';
}

// Format date to readable string
function formatDate(dateString) {
  const date = new Date(dateString);
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  });
}

// Show loading state
function showLoading() {
  isLoading = true;
  loadingEl.style.display = 'flex';
  errorEl.style.display = 'none';
  releasesContainerEl.style.display = 'none';
  emptyStateEl.style.display = 'none';
  refreshButtonEl.disabled = true;
}

// Show error state
function showError(message) {
  isLoading = false;
  loadingEl.style.display = 'none';
  errorEl.style.display = 'flex';
  releasesContainerEl.style.display = 'none';
  emptyStateEl.style.display = 'none';
  errorMessageEl.textContent = message;
  refreshButtonEl.disabled = false;
}

// Show releases
function showReleases(releasesData, markAsNew = false) {
  isLoading = false;
  
  // If markAsNew is true, detect new releases
  if (markAsNew && releases.length > 0) {
    const currentIds = new Set(releases.map(r => `${r.project_path}-${r.tag_name}`));
    releasesData.forEach(release => {
      const releaseId = `${release.project_path}-${release.tag_name}`;
      if (!currentIds.has(releaseId)) {
        newReleaseIds.add(releaseId);
      }
    });
  }
  
  releases = releasesData;
  
  loadingEl.style.display = 'none';
  errorEl.style.display = 'none';
  refreshButtonEl.disabled = false;
  
  if (releases.length === 0) {
    releasesContainerEl.style.display = 'none';
    emptyStateEl.style.display = 'flex';
  } else {
    releasesContainerEl.style.display = 'flex';
    emptyStateEl.style.display = 'none';
    renderReleases();
  }
  
  // Update last updated time
  lastUpdatedEl.textContent = `Last updated: ${new Date().toLocaleTimeString()}`;
}

// Parse tag name to extract attribute and version
function parseTagName(tagName) {
  if (!tagName) return { attribute: 'N/A', version: 'N/A' };
  
  // Look for version pattern (vX.X.X or similar)
  const versionMatch = tagName.match(/v\d+\.\d+(?:\.\d+)?(?:-\w+)?$/i);
  
  if (versionMatch) {
    const version = versionMatch[0];
    const prefix = tagName.slice(0, tagName.lastIndexOf(version));
    
    if (prefix && prefix.length > 0) {
      // Remove trailing dashes or underscores
      const attribute = prefix.replace(/[-_]+$/, '');
      return { 
        attribute: attribute || 'N/A', 
        version: version 
      };
    } else {
      return { 
        attribute: 'N/A', 
        version: version 
      };
    }
  } else {
    // No version pattern found, treat whole tag as version
    return { 
      attribute: 'N/A', 
      version: tagName 
    };
  }
}

// Render releases in the list
function renderReleases() {
  releasesListEl.innerHTML = '';
  
  releases.forEach(release => {
    const releaseItem = document.createElement('div');
    const releaseId = `${release.project_path}-${release.tag_name}`;
    const isNew = newReleaseIds.has(releaseId);
    
    releaseItem.className = isNew ? 'release-item new-release' : 'release-item';
    releaseItem.setAttribute('data-url', release.web_url);
    releaseItem.setAttribute('data-release-id', releaseId);
    
    const createdDate = new Date(release.created_at);
    const { attribute, version } = parseTagName(release.tag_name);
    
    releaseItem.innerHTML = `
      <div class="project-info">
        <div class="project-name">${release.project_name}</div>
      </div>
      <div class="release-info">
        <div class="release-name">${release.name || 'Unnamed Release'}</div>
      </div>
      <div class="attribute-info">
        ${attribute === 'N/A' ? '<span class="attribute-na">N/A</span>' : `<span class="attribute-name">${attribute}</span>`}
      </div>
      <div class="version-info">
        <span class="tag-name">${version}</span>
      </div>
      <div class="date-info">
        <span class="date-time">${formatDate(release.created_at)}</span>
        <span class="date-relative">${formatRelativeTime(createdDate)}</span>
      </div>
    `;
    
    // Add hover handler to remove new release highlighting
    if (isNew) {
      releaseItem.addEventListener('mouseenter', async () => {
        releaseItem.classList.remove('new-release');
        newReleaseIds.delete(releaseId);
        
        // If no more new releases, clear the notification
        if (newReleaseIds.size === 0) {
          try {
            await invoke("mark_releases_as_seen");
          } catch (error) {
            console.error("Failed to mark releases as seen:", error);
          }
        }
      });
    }
    
    // Add click handler to open release URL
    releaseItem.addEventListener('click', () => {
      openReleaseUrl(release.web_url);
    });
    
    releasesListEl.appendChild(releaseItem);
  });
}

// Open release URL
async function openReleaseUrl(url) {
  try {
    await invoke("open_release_url", { url });
  } catch (error) {
    console.error("Failed to open URL:", error);
  }
}



// Refresh releases
async function refreshReleases() {
  if (isLoading) return;
  
  showLoading();
  
  try {
    const newReleases = await invoke("refresh_releases");
    showReleases(newReleases);
    console.log("Releases refreshed successfully");
  } catch (error) {
    console.error("Failed to refresh releases:", error);
    showError(`Failed to refresh releases: ${error}`);
  }
}

// Load initial releases
async function loadReleases() {
  showLoading();
  
  try {
    const releasesData = await invoke("get_releases");
    showReleases(releasesData);
    console.log("Initial releases loaded");
  } catch (error) {
    console.error("Failed to load releases:", error);
    showError(`Failed to load releases: ${error}`);
  }
}

// Handle keyboard shortcuts
function handleKeyboard(event) {
  // Refresh on 'R' key
  if (event.key.toLowerCase() === 'r' && !event.ctrlKey && !event.altKey && !event.metaKey) {
    event.preventDefault();
    refreshReleases();
  }
}

// Initialize the application
window.addEventListener("DOMContentLoaded", async () => {
  // Get DOM elements
  refreshButtonEl = document.querySelector("#refresh-button");
  retryButtonEl = document.querySelector("#retry-button");
  releasesListEl = document.querySelector("#releases-list");
  loadingEl = document.querySelector("#loading");
  errorEl = document.querySelector("#error");
  errorMessageEl = document.querySelector("#error-message");
  releasesContainerEl = document.querySelector("#releases-container");
  emptyStateEl = document.querySelector("#empty-state");
  lastUpdatedEl = document.querySelector("#last-updated");
  
  // Add event listeners
  refreshButtonEl.addEventListener("click", refreshReleases);
  retryButtonEl.addEventListener("click", refreshReleases);
  
  // Intercept the native window close action and hide the window instead of quitting
  appWindow.onCloseRequested(({ preventDefault }) => {
    preventDefault();
    hideToTray();
  });
  
  // Add keyboard event listener
  document.addEventListener("keydown", handleKeyboard);
  
  // Listen for Tauri events
  await listen("releases-loaded", (event) => {
    console.log("Releases loaded event received");
    showReleases(event.payload, false); // Initial load, don't mark as new
  });
  
  await listen("releases-updated", (event) => {
    console.log("Releases updated event received");
    showReleases(event.payload, true); // Updated releases, mark new ones
  });
  
  // Load initial releases
  await loadReleases();
  
  console.log("GitLab Releases Monitor initialized");
});
