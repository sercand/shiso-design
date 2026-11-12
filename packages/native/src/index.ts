export * from "./tokens";
export * from "./theme";
export * from "./components";

// Metro turns a font require into an asset id.
declare const require: (path: string) => number;

/** Pass to expo-font's useFonts (or link natively). Keys are the family names the components use. */
export const fontAssets = {
  "HankenGrotesk-Regular": require("../../../fonts/HankenGrotesk-Regular.ttf"),
  "HankenGrotesk-Medium": require("../../../fonts/HankenGrotesk-Medium.ttf"),
  "HankenGrotesk-SemiBold": require("../../../fonts/HankenGrotesk-SemiBold.ttf"),
  "HankenGrotesk-Bold": require("../../../fonts/HankenGrotesk-Bold.ttf"),
  "JetBrainsMono-Regular": require("../../../fonts/JetBrainsMono-Regular.ttf"),
  "JetBrainsMono-Medium": require("../../../fonts/JetBrainsMono-Medium.ttf"),
} as const;
