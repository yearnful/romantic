/*
 * romantic, a Discord client mod
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

import { addServerListElement, removeServerListElement, ServerListItem, ServerListRenderPosition } from "@api/ServerList";
import { definePluginSettings } from "@api/Settings";
import { Button } from "@components/Button";
import ErrorBoundary from "@components/ErrorBoundary";
import { EquicordDevs } from "@utils/constants";
import definePlugin, { OptionType, PluginNative } from "@utils/types";
import { React } from "@webpack/common";

const Native = VencordNative.pluginHelpers.RomanceDashboard as PluginNative<typeof import("./native")>;

const settings = definePluginSettings({
    start: {
        type: OptionType.SELECT,
        description: "Page to open first",
        options: [
            { label: "Dashboard", value: "/dashboard", default: true },
            { label: "Analytics", value: "/dashboard#analytics" },
            { label: "Links", value: "/dashboard#links" },
            { label: "Look", value: "/dashboard#look" },
            { label: "Settings", value: "/dashboard#settings" }
        ]
    }
});

function open() {
    return Native.openRomance(settings.store.start);
}

function HeartIcon({ className, width = 24, height = 24 }: { className?: string; width?: number; height?: number; }) {
    return (
        <svg className={className} width={width} height={height} viewBox="0 0 7 6" shapeRendering="crispEdges">
            <path fill="currentColor" d="M1 0h2v1H1zM4 0h2v1H4zM0 1h7v2H0zM1 3h5v1H1zM2 4h3v1H2zM3 5h1v1H3z" />
        </svg>
    );
}

function ServerButton() {
    return (
        <ServerListItem
            id="romantic-romance-button"
            icon={HeartIcon}
            tooltip="romance.rip"
            onClick={open}
        />
    );
}

export default definePlugin({
    name: "RomanceDashboard",
    description: "Opens your romance.rip dashboard in its own window from the toolbox. Your sign-in is kept between launches.",
    authors: [EquicordDevs.nobody],
    settings,
    start() {
        addServerListElement(ServerListRenderPosition.Above, ServerButton);
    },
    stop() {
        removeServerListElement(ServerListRenderPosition.Above, ServerButton);
    },
    toolboxActions: {
        "Open romance.rip dashboard": open
    },
    settingsAboutComponent: ErrorBoundary.wrap(() => <Button onClick={open}>Open romance.rip dashboard</Button>, { noop: true })
});
