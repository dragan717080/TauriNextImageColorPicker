## <a name="no-link"></a> Tauri + Next.js Image Color Picker

Image color picker built with Next.js, Tauri and Python.
This app uses **pnpm** as the Node.js dependency
manager. Performant thanks to Rust on the backend, good looking
with Next.js/Tailwind/Shadcn. Also you need Rust to run this app.

https://github.com/dragan717080/TauriNextImageColorPicker/assets/135660124/4ab6ed16-1db3-469b-b467-732a984cff92


## <a name="no-link"></a>Technologies Used

- **Next.js**

Next.js is a React framework for building modern web applications. It provides server-side rendering (SSR) and static site generation (SSG) capabilities, resulting in faster page loads and improved SEO. Next.js simplifies the development process and offers features like automatic code splitting, routing, and hot module replacement.

- **Tauri**

Tauri is a Rust based lightweight and flexible framework designed for building desktop applications with support for web technologies like server-side rendering (SSR) and static site generation (SSG). It creates modern, performant desktop applications.

- **TypeScript**

TypeScript is a strongly typed superset of JavaScript that enhances code maintainability and scalability. It allows us to catch errors during development and provides better tooling support, leading to more robust applications.

- **Tailwind CSS**

Tailwind CSS is a utility-first CSS framework that enables rapid UI development. Its utility classes make it easy to create responsive and custom-designed user interfaces without writing custom CSS.

- **Python**

Backend scripting language.

## <a name="no-link"></a>Features

### <a name="no-link"></a>Upload Image and see colors for selected pixel
Display hex and RGB code for selected pixel on uploaded image. It uses Rust with Python on backend to process selected image and return info.

## <a name="no-link"></a>Getting Started

### Running development server and use Tauri window

After cloning for the first time, set up git pre-commit hooks:

```shell
pnpm install husky --save-dev
```

To develop and run the frontend in a Tauri window:

```shell
pnpm dev
```

This will load the Next.js frontend directly in a Tauri webview window.

### Source structure

Next.js frontend source files are located in `src/` and Tauri Rust application source
files are located in `src-tauri/`.
