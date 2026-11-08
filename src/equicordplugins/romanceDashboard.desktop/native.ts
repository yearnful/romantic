/*
 * romantic, a Discord client mod
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

import { BrowserWindow, IpcMainInvokeEvent, shell } from "electron";

let win: BrowserWindow | null = null;

export function openRomance(_: IpcMainInvokeEvent, path: string) {
    const url = new URL(path || "/dashboard", "https://romance.rip");
    if (url.hostname !== "romance.rip") throw new Error("Unknown URL");

    if (win && !win.isDestroyed()) {
        win.loadURL(url.toString());
        win.show();
        win.focus();
        return;
    }

    win = new BrowserWindow({
        width: 1240,
        height: 820,
        minWidth: 720,
        minHeight: 520,
        backgroundColor: "#060606",
        autoHideMenuBar: true,
        title: "romance.rip",
        webPreferences: {
            partition: "persist:romantic-romance",
            contextIsolation: true,
            nodeIntegration: false,
            sandbox: true
        }
    });

    win.webContents.setWindowOpenHandler(({ url }) => {
        if (new URL(url).hostname === "romance.rip") {
            win?.loadURL(url);
        } else if (/^https?:$/.test(new URL(url).protocol)) {
            shell.openExternal(url);
        }
        return { action: "deny" };
    });
    win.on("closed", () => { win = null; });
    win.loadURL(url.toString());
}
