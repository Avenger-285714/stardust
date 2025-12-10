# Stardust Design Document

## UI Layout

The Stardust application features a modern, clean interface organized as follows:

### Main Window

```
┌─────────────────────────────────────────────────────────────┐
│ Stardust App Store                                          │
├─────────────────────────────────────────────────────────────┤
│ [Search applications...                                   ] │
├─────────────────────────────────────────────────────────────┤
│ [All] [Development] [Graphics] [Office] [Games]            │
│ [Multimedia] [Network] [Utilities]                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Browsing: All                                               │
│                                                             │
│ Welcome to Stardust!                                        │
│                                                             │
│ This is an experimental app store built with Rust and Iced. │
│ Start by searching for applications or browsing categories. │
│                                                             │
│                                                             │
│                    (Scrollable Content Area)                │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Components

### 1. Title Bar
- Displays "Stardust App Store"
- Large, prominent text (28px)

### 2. Search Bar
- Full-width text input
- Placeholder: "Search applications..."
- Real-time search query updates

### 3. Category Navigation
- Horizontal row of category buttons
- Categories:
  - All (default)
  - Development
  - Graphics
  - Office
  - Games
  - Multimedia
  - Network
  - Utilities
- Visual feedback on selection

### 4. Content Area
- Scrollable region for displaying applications
- Currently shows welcome message and category information
- Dynamically updates based on search and category selection

## Architecture

### State Management (Stardust struct)
```rust
struct Stardust {
    search_query: String,        // Current search text
    selected_category: Category, // Active category filter
}
```

### Message Types
- `SearchChanged(String)` - User types in search bar
- `CategorySelected(Category)` - User clicks category button
- `AppSelected(String)` - User selects an application (future)

### Update Flow
1. User interaction generates a Message
2. Message is passed to update() method
3. State is modified based on message
4. View is automatically re-rendered with new state

## Color Scheme

The application uses Iced's default theme, which provides:
- Clean, modern appearance
- Good contrast for readability
- Consistent spacing and padding

## Future Enhancements

1. **Application Cards**
   - Icon display
   - Name and description
   - Version information
   - Install button

2. **Details View**
   - Full application description
   - Screenshots
   - Reviews and ratings
   - Installation status

3. **Package Integration**
   - Connect to system package managers
   - Download and install functionality
   - Update notifications

4. **Settings**
   - Theme customization
   - Update preferences
   - Repository configuration

5. **Search Improvements**
   - Fuzzy matching
   - Tag-based filtering
   - Sort options (popularity, name, date)
