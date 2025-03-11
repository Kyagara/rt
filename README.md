# RT (name pending)

A Twitch frontend written in Rust using SvelteKit and Tauri.

<table>
    <tr>
        <td align="center">
            <img alt="zfg1 playing OOT Randomizer" src=".github/assets/screenshot-1.jpg" width="500">
            <em>
                <div>
                    <a href="https://www.twitch.tv/zfg1">zfg1</a> playing OOT. Chat on Copium for Prime 4.
                </div>
            </em>
        </td>
        <td align="center">
            <img alt="paganmars playing Monster Hunter Wilds" src=".github/assets/screenshot-2.jpg" width="500">
            <em>
                <div>
                    <a href="https://www.twitch.tv/paganmars">paganmars</a> playing Monster Hunter Wilds. An excelent use of screen space with PiP.
                </div>
            </em>
        </td>
    </tr>
</table>

> This is not meant to be a replacement for the official Twitch app/site, some features (account login, send chat messages, etc) are not implemented and not in the scope of this project.

## Features

- Watch streams in any of the available resolutions.
- View chat with 7tv and BetterTTV emotes support.
- WIP ad blocking.
- Get basic information such as viewer count, stream title, current game, hours streaming.
- Replace `https` with `rt` in a stream URL to open the app and save the user.

## Download

> Only installers are available for now and all (`exe`, `deb`, `rpm`) are provided in the same zip file.

[Quick download.](https://nightly.link/Kyagara/rt/workflows/build.yaml/main/bundles.zip)

Github Actions builds are available [here](https://github.com/Kyagara/rt/actions).

## About

> Tested on Windows, other desktops should work but haven't been tested.

- Frontend `src`: [SvelteKit](https://svelte.dev/docs/kit/introduction), [tailwindcss](https://tailwindcss.com).
- Backend `src-tauri`: [Tauri](https://tauri.app/).

Data is stored in the following locations:

- Windows: `%AppData%/Roaming/com.rt.app`
- Linux: `~/.config/com.rt.app`

Logs are stored in the following locations:

- Linux: `$XDG_DATA_HOME/com.rt.app/logs` or `$HOME/.local/share/com.rt.app/logs`
- Windows: `%LocalAppData%/com.rt.app/logs`

## TODO

- Add global Twitch emotes and fetch the user emotes.
- Add persistent settings (save stream resolution, volume, etc).
- More helpers like getting a string from a json pointer.
- Try rt on other desktops.
- Somehow make proxy work with `invoke`/another method that doesn't require a local server, might require parsing the m3u8 in the frontend.
- Use Tauri's channels to notify more events, like when the stream has been switched to a backup one and more.
- Try to reduce as much as possible the amount of requests made to Twitch.
- Fix `join_chat` errors when hard refreshing the page while in a chat.
