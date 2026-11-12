import React, { createContext, useContext, useMemo } from "react";
import { useColorScheme } from "react-native";
import { colors, density as densities, fonts, radius, space, text, type Density, type Mode } from "./tokens";

export type Theme = {
  mode: Mode;
  /** Colors of the current mode. */
  c: (typeof colors)[Mode];
  /** Sizes of the current density. */
  d: (typeof densities)[Density];
  fonts: typeof fonts;
  radius: typeof radius;
  space: typeof space;
  text: typeof text;
  /** Card elevation as a style, already right for the mode. */
  cardShadow: { boxShadow: string };
};

function build(mode: Mode, density: Density): Theme {
  const c = colors[mode];
  const boxShadow =
    mode === "dark"
      ? "0 0 0 1px rgba(255,255,255,0.05)"
      : "0 1px 2px rgba(16,25,36,0.06), 0 10px 28px -14px rgba(16,25,36,0.16)";
  return { mode, c, d: densities[density], fonts, radius, space, text, cardShadow: { boxShadow } };
}

const Ctx = createContext<Theme>(build("light", "comfortable"));

/**
 * Put one provider at the root. `mode="system"` follows the device.
 * Density defaults to comfortable: touch targets need the room.
 */
export function ShisoProvider(props: { mode?: Mode | "system"; density?: Density; children: React.ReactNode }) {
  const system = useColorScheme();
  const mode: Mode = !props.mode || props.mode === "system" ? (system === "dark" ? "dark" : "light") : props.mode;
  const density = props.density ?? "comfortable";
  const value = useMemo(() => build(mode, density), [mode, density]);
  return <Ctx.Provider value={value}>{props.children}</Ctx.Provider>;
}

export const useShiso = () => useContext(Ctx);
