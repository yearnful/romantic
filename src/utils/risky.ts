export const RISKY_PLUGINS: Record<string, string> = {
    scheduledmessages: "Sends messages on a timer. Automated sending can look like a self-bot to Discord.",
    repeatmessages: "Sends the same message again and again. Discord can flag this as spam or automation.",
    autojumptomessage: "Acts on messages for you without a click. Automation of this kind can be flagged.",
    waitforslot: "Polls Discord in the background to grab a slot. Repeated automated requests can be flagged.",
    signature: "Edits every message you send. Message tampering can be flagged.",
    universalmention: "Sends mentions in bulk. Mass mentions can get an account limited.",
    nopushtotalk: "Changes how voice input is sent. Client behavior Discord does not expect.",
    fakenitro: "Fakes Nitro features with links and emoji tricks. This breaks Discord's terms.",
    spotifycrack: "Bypasses Spotify listen-along limits. This breaks Discord's and Spotify's terms.",
    showhiddenchannels: "Requests channels you were not shown. Unusual requests can be flagged.",
    unlimitedaccounts: "Lets you hold more accounts than Discord allows. This breaks Discord's terms.",
    nonitroupsell: "Hides Nitro prompts. Low risk, but it changes client behavior.",
    norpc: "Blocks rich presence events. Low risk, but it changes client behavior."
};

export function riskOf(name: string): string | undefined {
    return RISKY_PLUGINS[name.toLowerCase()];
}
