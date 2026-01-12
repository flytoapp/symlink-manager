# Symlink Manager User Manual

A cross-platform desktop application for managing symbolic links across multiple directories.

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Getting Started](#getting-started)
- [Core Concepts](#core-concepts)
- [User Interface](#user-interface)
- [Managing Profiles](#managing-profiles)
- [Managing Sources](#managing-sources)
- [Managing Items](#managing-items)
- [Settings](#settings)
- [Platform-Specific Notes](#platform-specific-notes)
- [Use Cases](#use-cases)

---

## Overview

Symlink Manager helps you organize and toggle symbolic links through a visual interface. Instead of manually creating and removing symlinks via the command line, you can enable or disable them with a single click.

The application uses a hierarchical organization:
- **Profiles** group related symlink configurations
- **Sources** define where your files are and where symlinks should be created
- **Items** are the individual files or folders that can be symlinked

---

## Installation

Download the latest release for your platform from the [Releases](https://github.com/flytoapp/symlink-manager/releases) page:

| Platform | File Type |
|----------|-----------|
| Windows  | `.msi` or `.exe` |
| macOS    | `.dmg` |
| Linux    | `.deb` or `.AppImage` |

---

## Getting Started

1. **Launch the application** - On first launch, you'll see an empty workspace with a prompt to create a profile.

2. **Create a profile** - Click **+ New Profile**, enter a name and select a default target folder where symlinks will be created.

3. **Add a source** - With your profile selected, click **+ Add Source** to add a folder containing items you want to manage.

4. **Enable items** - Click on individual items to create symlinks, or click again to remove them.

---

## Core Concepts

### Profiles

A profile is a named configuration that groups related symlink sources together. Each profile has:

- **Name** - A descriptive label (e.g., "X-Plane 12", "Dotfiles")
- **Base Path** - The default target directory where symlinks will be created

Profiles are useful for organizing different projects or applications that need symlink management.

### Sources

A source defines a directory containing items you want to symlink. Each source has:

- **Name** - A descriptive label (e.g., "Plugins", "Aircraft")
- **Source Path** - The folder containing your actual files
- **Target Path** (optional) - Where symlinks will be created. If not specified, uses the profile's base path.

### Items

Items are the individual files and folders within a source directory. Each item can be:

- **Enabled** - A symlink exists pointing to the source item
- **Disabled** - No symlink exists

---

## User Interface

![Profile View](/profile.png)

The interface is divided into three main areas:

### Left Sidebar

- **Profiles section** - Lists all profiles with buttons to create, edit, and delete
- **Sources section** - Shows sources for the selected profile with edit and delete options
- **Settings button** - Access application settings

### Main Content Area

The content area changes based on your selection:

- **No profile selected** - Welcome message with getting started prompt
- **Profile selected (no source)** - Profile summary showing all active items across sources
- **Source selected** - Item list with filtering, search, and toggle controls

### Profile Summary View

When a profile is selected but no source is active, you see a summary of all active symlinks organized by source. This gives you a quick overview of what's currently enabled.

![Source View](/source.png)

### Source/Items View

When a source is selected, you see:

- **Source and Target paths** - Clickable to open in your file manager
- **Filter tabs** - All, Active, or Inactive items
- **Search box** - Fuzzy search to find items quickly
- **Item list** - Click items to toggle their symlink status
- **Status legend** - Shows what each color means

---

## Managing Profiles

### Creating a Profile

1. Click **+ New Profile** in the Profiles section
2. Enter a **Profile Name** (e.g., "Game Mods")
3. Select the **Default Target Folder** - this is where symlinks will be created by default
4. Click **Create Profile**

### Editing a Profile

1. Click the **pencil icon** next to the profile you want to edit
2. Modify the name or base path
3. Click **Save Changes**

### Deleting a Profile

1. Click the **trash icon** next to the profile you want to delete
2. Confirm the deletion in the dialog

> **Warning**: Deleting a profile will remove all symlinks managed by that profile.

### Selecting a Profile

Click on any profile in the list to select it. The selected profile is highlighted with a blue border.

---

## Managing Sources

### Creating a Source

1. Select a profile first
2. Click **+ Add Source** in the Sources section
3. Enter a **Source Name** (e.g., "Texture Mods")
4. Select the **Source Folder** containing your files
5. Optionally enable **Use custom target folder** if symlinks should go somewhere other than the profile's base path
6. Click **Add Source**

### Editing a Source

1. Click the **pencil icon** next to the source you want to edit
2. Modify the name, source path, or target path
3. Click **Save Changes**

### Deleting a Source

1. Click the **trash icon** next to the source you want to delete
2. Confirm the deletion in the dialog

> **Warning**: Deleting a source will remove all symlinks from that source.

### Source Display

Each source card shows:
- **Name** - The source's display name
- **Source path** - Where the actual files are located (folder icon)
- **Target path** - Where symlinks will be created (arrow icon)

---

## Managing Items

### Item List

When you select a source, the main area shows all files and folders in that source directory.

### Enabling/Disabling Items

- **Click an item** to toggle its symlink status
- **Green background with checkmark** = Active (symlink exists)
- **Gray background** = Inactive (no symlink)
- **Red background** = Conflict (explained below)

### Filtering Items

Use the filter tabs above the item list:
- **All** - Shows all items
- **Active** - Shows only items with active symlinks
- **Inactive** - Shows only items without symlinks

Each tab shows a count of matching items.

### Searching Items

Type in the search box to filter items by name. The search uses fuzzy matching, so you don't need to type the exact name.

### Disable All

When there are active items, a **Disable All** button appears. Click it to remove all symlinks in the current source at once.

### Refresh

Click the **Refresh** button to reload the item list from disk. Use this if you've made changes outside the application.

### Context Menu

Right-click any item to access the context menu:
- **Reveal in Finder/Explorer/Files** - Opens the file manager and selects the item

### Item Status Colors

| Color | Status | Meaning |
|-------|--------|---------|
| Green | Active | Symlink exists and points to this item |
| Gray | Inactive | No symlink exists, or symlink is broken |
| Red | Conflict | A regular file/folder already exists at the target location |

### Conflicts

A conflict occurs when a regular file or folder (not a symlink) already exists at the target path with the same name as your source item.

Conflicting items cannot be toggled until the conflict is resolved manually by removing or renaming the existing file/folder at the target location.

### Broken Symlinks

If a symlink exists but points to a location that no longer exists, it appears as inactive (gray). You can click it to recreate the symlink correctly.

---

## Settings

Access settings by clicking the **Settings** button at the bottom of the sidebar.

### Theme

Choose your preferred appearance:
- **Light** - Light background with dark text
- **Dark** - Dark background with light text
- **System** - Automatically matches your operating system's theme

---

## Platform-Specific Notes

### Windows

Creating symbolic links on Windows requires elevated permissions. You have two options:

1. **Enable Developer Mode** (Recommended)
   - Open Windows Settings
   - Navigate to **System > For developers** (Windows 11) or **Update & Security > For developers** (Windows 10)
   - Enable "Developer Mode"
   - This allows symlink creation without administrator privileges

2. **Run as Administrator**
   - Right-click the application and select "Run as administrator"
   - You'll need to do this each time you launch the app

If symlink permissions aren't available, a warning banner appears in the items view explaining how to enable them.

### macOS

No special permissions required. Symlinks work out of the box.

### Linux

No special permissions required for most desktop Linux distributions.

---

## Use Cases

### Game Mod Management

Organize mods for games like X-Plane, Flight Simulator, or others that use folder-based mod systems:

1. Create a profile for the game (e.g., "X-Plane 12")
2. Set the base path to the game's mod/addon folder
3. Add sources for different mod categories (Plugins, Aircraft, Scenery)
4. Enable/disable mods as needed without moving files

### Dotfiles Management

Symlink configuration files from a dotfiles repository:

1. Create a profile for your dotfiles
2. Set the base path to your home directory or `~/.config`
3. Add your dotfiles repository folder as a source
4. Selectively enable the configurations you need

### Development Environment

Share configuration between projects:

1. Create a profile for shared configs
2. Add sources for different config types (ESLint, Prettier, TypeScript)
3. Enable the configs you need for each project

### Plugin Development

Test plugins during development:

1. Create a profile for the host application
2. Add your plugin development folder as a source
3. Toggle the symlink to test your plugin without manual copying

---

## Troubleshooting

### Items not appearing

- Click the **Refresh** button to reload from disk
- Verify the source path is correct by clicking the path to open it

### Cannot create symlinks on Windows

- Enable Developer Mode in Windows Settings, or
- Run the application as Administrator

### Conflict status on items

- Check what exists at the target location
- Manually remove or rename the conflicting file/folder
- Click Refresh to update the status

### Changes not reflected

- The application caches item status for performance
- Click **Refresh** to reload the current source
- Re-select the source to force a full reload
