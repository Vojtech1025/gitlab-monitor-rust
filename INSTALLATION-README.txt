GitLab Releases Monitor - Installation Complete
================================================

Thank you for installing GitLab Releases Monitor!

IMPORTANT: CONFIGURATION REQUIRED BEFORE FIRST USE
==================================================

To start monitoring your GitLab projects, you need to configure the application:

1. LOCATE THE CONFIGURATION FILES
   --------------------------------
   In the same directory where the application was installed, you'll find:
   - "gitlab-config.example" (configuration template)
   - "INSTALLATION-README.txt" (this file)
   - "gitlab-monitor.exe" (the application)

2. CREATE YOUR .env FILE
   ----------------------
   COPY the "gitlab-config.example" file and RENAME it to ".env"
   
   Windows: Right-click "gitlab-config.example" → Copy → Paste → Rename to ".env"
   
   IMPORTANT: The file MUST be named exactly ".env" (including the dot at the beginning)

3. EDIT YOUR .env FILE
   -------------------
   Open the ".env" file with any text editor (like Notepad) and replace the 
   placeholder values with your actual GitLab information:

   GITLAB_API_TOKEN=<your-api-token>
   GITLAB_BASE_URL=<your-base-url>
   GITLAB_PROJECTS=<your-projects>

   Example configuration:
   GITLAB_API_TOKEN=glpat-xxxxxxxxxxxxxxxxxxxx
   GITLAB_BASE_URL=https://gitlab.com
   GITLAB_PROJECTS=mygroup/project1,mygroup/project2

4. GET YOUR GitLab API TOKEN
   -------------------------
   - Go to your GitLab instance (e.g., https://gitlab.com)
   - Navigate to: User Settings → Access Tokens
   - Create a new token with "read_api" scope
   - Copy the token and use it as GITLAB_API_TOKEN

5. SPECIFY YOUR PROJECTS
   ----------------------
   List your projects in the format: group/project1,group/project2
   Example: mycompany/backend,mycompany/frontend,john.doe/myproject

6. START THE APPLICATION
   ---------------------
   After creating and editing the .env file, launch the application.
   It will appear in your system tray (bottom-right corner of your screen).

FILE SUMMARY
============
- gitlab-config.example → COPY and RENAME to ".env", then EDIT
- .env → Your actual configuration file (create from the example)
- INSTALLATION-README.txt → These instructions
- gitlab-monitor.exe → The application

TROUBLESHOOTING
===============
- If the application doesn't start, check that the .env file exists and is properly configured
- The application will show helpful error messages if configuration is incomplete
- Make sure the .env file is in the same directory as the executable
- Check the console output for detailed configuration instructions

USAGE
=====
- Left-click the tray icon to show/hide the window
- Right-click the tray icon for menu options
- The tray icon turns blue when new releases are detected
- Press Alt+G to toggle the window (global shortcut)

For more information and support, visit:
https://github.com/Vojtech1025/gitlab-monitor-rust

Enjoy monitoring your GitLab releases! 