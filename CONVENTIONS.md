# Documentation and Conventions

## Introduction

This project is a GUI application built using the `iced` crate, which provides a reactive framework for building cross-platform applications in Rust. The application is designed to be modular and maintainable, with a focus on clear code organization and styling.

## Code Style Guide

### Naming Conventions

- **Types**: Use `CamelCase` for struct, enum, and trait names.
- **Variables and Functions**: Use `snake_case` for variable and function names.
- **Constants**: Use `UPPER_SNAKE_CASE` for constants.

### Module Organization

- Group related functionalities into modules.
- Use `mod.rs` for module definitions to keep the codebase organized.

### Comments and Documentation

- Use `///` for public API documentation to generate documentation with `cargo doc`.
- Use `//` for inline comments to explain complex logic or important decisions.

## Using the `iced` Crate

### Subscriptions and Event Handling

- **iced::Subscription**: Used to handle asynchronous events and tasks. Subscriptions are created to listen for events such as time intervals, window events, or custom events like URL changes. They are essential for integrating asynchronous operations into the application's update loop.
- **iced::advanced::subscription**: Provides more advanced subscription capabilities, allowing for custom event streams and more complex event handling logic. This is useful for scenarios where the default subscription mechanisms are insufficient.
- **iced::daemon**: Used to run the application in a daemon mode, which is suitable for long-running processes that need to handle events continuously. This is particularly useful for applications that need to maintain a persistent state or connection.

### Event Handling

- **Event Propagation**: Events are propagated through the application using the `update` method, which processes messages and updates the application state accordingly.
- **Window Events**: The application listens for window events such as resizing, moving, and focus changes. These events are handled to update the UI layout and maintain application state consistency.
- **Custom Events**: Custom events can be defined and handled to trigger specific actions within the application, such as opening a new window or changing themes.

- **iced::Subscription**: Used to handle asynchronous events and tasks. Subscriptions are created to listen for events such as time intervals, window events, or custom events like URL changes. They are essential for integrating asynchronous operations into the application's update loop.
- **iced::advanced::subscription**: Provides more advanced subscription capabilities, allowing for custom event streams and more complex event handling logic. This is useful for scenarios where the default subscription mechanisms are insufficient.
- **iced::daemon**: Used to run the application in a daemon mode, which is suitable for long-running processes that need to handle events continuously. This is particularly useful for applications that need to maintain a persistent state or connection.

### Basic Structure

- **Application**: The main entry point of the application, implementing the `iced::Application` trait.
- **Message**: An enum used to define all possible events and actions in the application.
- **Element**: Represents a UI component, created using various widgets.

### Widgets

- Always prioritize using the built-in widgets provided by the `iced` crate before creating custom widgets. The built-in widgets are optimized for performance and integration with the `iced` framework.
- The `iced` crate provides a variety of built-in widgets that should be prioritized for use. These include:
  - `Button`: Allows users to perform actions by pressing them.
  - `Canvas`: Can be leveraged to draw interactive 2D graphics.
  - `Checkbox`: Used to let users make binary choices.
  - `ComboBox`: Displays a dropdown list of searchable and selectable options.
  - `Container`: Lets you align a widget inside their boundaries.
  - `Image`: Displays raster graphics in different formats (PNG, JPG, etc.).
  - `Markdown`: Parses and displays Markdown.
  - `PaneGrid`: Lets users split regions of your application and organize layout dynamically.
  - `PickList`: Displays a dropdown list of selectable options.
  - `ProgressBar`: Visualizes the progression of an extended computer operation, such as a download, file transfer, or installation.
  - `Radio`: Lets users choose a single option from a bunch of options.
  - `Scrollable`: Lets users navigate an endless amount of content with a scrollbar.
  - `Slider`: Lets users set a value by moving an indicator.
  - `Svg`: Displays vector graphics in your application.
  - `Text`: Draws and interacts with text.
  - `TextInput`: Displays fields that can be filled with text.
  - `Toggler`: Lets users make binary choices by toggling a switch.
  - `Tooltip`: Displays a hint of information over some element when hovered.

### State Management

- Use `State` structs to manage the state of different components.
- The `Message` enum is used to handle events and update the state accordingly.

## Theming and Styling

### Theme Implementation

- Themes are defined using the `Theme` struct, which implements the `iced::theme::Base` trait.
- Use `Style` structs to customize the appearance of widgets.

### Custom Widgets

- Create custom widgets by implementing the `Widget` trait.
- Integrate custom widgets with the `iced` framework by defining their layout, style, and behavior.

## Best Practices

### Dependency Management

- Always use the latest stable versions of crates to ensure compatibility and access to the latest features and security updates.
- Avoid using code that requires downgrading crates, as this can lead to compatibility issues and missing out on important updates.
- Regularly update dependencies and test the application to ensure it works with the latest versions.

### Performance Considerations

- Optimize rendering by minimizing state changes and using efficient data structures.
- Use `iced::Subscription` for handling asynchronous tasks without blocking the UI.

### Error Handling

- Use `Result` and `Option` types for error handling.
- Log errors using the `log` crate to keep track of issues during development and production.

## Contributing

- Follow the code style guide and conventions outlined in this document.
- Ensure all code is tested and reviewed before merging.
- Use `cargo fmt` and `cargo clippy` to maintain code quality.
