# konsol-2024 (WIP)
KONSol is an information screen in Konsulatet. This repo contains the React app that is the screen itself, a React admin page to upload slides, and the backend written in Actix Web.

## Features:
- Ability to upload slides (images, title, text, date) that will be shown on the screen.
- SL timetable data.
- Fysiksektionen calendar integration.

## Backend config
- **Environment Variables:**
    (Defined in `backend/.env`)
  - `DATABASE_URL`: Path to the SQLite database file.
  - `IMAGE_PATH`: Directory where slide images are stored. Defaults to `/tmp/konsol_slides`. Warning: Currently all files in this directory are served under `/api/screen/slides/images`.

## Endpoints
See [endpoints](endpoints.md).

## Development setup
### Backend
1. Make sure Rust is installed and `cargo` works. Otherwise, [install Rust](https://www.rust-lang.org/tools/install).
2. [Install Diesel CLI](https://diesel.rs/guides/getting-started), for example by running `cargo install diesel_cli` or `cargo-binstall diesel_cli`. 
3. Navigate to `backend/` and run `diesel migration run`.
4. To fill the database with 4 mock slides, navigate to `backend/scripts` and (while in that directory) run `python3 mock_data.py`.
5. Run the backend with `cargo run` or `cargo r` from the `backend/` directory. (The port will be displayed in the terminal)

### Frontends
1. Make sure `node` and `npm` are both installed.
2. Navigate to either `screen-frontend/` or `admin-frontend` and run `npm install`.
3. Run the frontend with `npm run dev`. (The port will be displayed in the terminal)

### Nix
If you don't use Nix, you can ignore this and all `*.nix`-files. If you use Nix, this project has a dev shell which can be entered with `nix develop` (if you use flakes) or `nix-shell` (if you don't). After entering the dev shell, run `npm install` to install the Node dependencies, run the migrations, and fill the database according to the instructions above.