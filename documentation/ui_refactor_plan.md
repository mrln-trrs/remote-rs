# Refactor UI: YT Music Pattern

## 1. Visual Hierarchy
- **Primary Background:** Deep Black (`#030303`).
- **Surface Color:** Dark Gray (`#121217`).
- **Accent:** Cyan/Teal (`#45d7ff`).
- **Typography:** Inter/Roboto, bold headers.

## 2. Layout Structure (App.svelte)
- **Content Area:** Dynamic scrollable area for pages.
- **Persistent Mini-Player:** A slim bar above the navigation tabs showing current track + Play/Pause button.
- **Bottom Navigation:** Fixed horizontal bar with icons for Home, Player, Controls, and System.
- **Player Overlay:** A full-screen view that slides up when the Mini-Player is tapped.

## 3. Interaction Design
- **One-Handed Navigation:** All tabs and primary controls are in the bottom 30% of the screen.
- **Glassmorphism:** Mini-player and tabs will have a subtle background blur (`backdrop-filter`).
- **Responsive Grids:** Use 2-column or 3-column grids for launchers on mobile to maximize space.

## 4. Proposed Component Changes
- `PageTabs.svelte`: Convert from grid to flex-row at the bottom.
- `MiniPlayer.svelte`: New component for persistent control.
- `HomePage.svelte`: Implement a "Quick Access" grid and categorized rows.
- `PlayerPage.svelte`: Redesign as a full-screen immersive view with a blurred background.
