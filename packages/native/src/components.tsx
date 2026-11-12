import React from "react";
import {
  Pressable,
  Switch as RNSwitch,
  Text as RNText,
  TextInput,
  View,
  type StyleProp,
  type TextInputProps,
  type TextProps,
  type TextStyle,
  type ViewStyle,
} from "react-native";
import { useShiso } from "./theme";

type Variant = "body" | "muted" | "small" | "label" | "strong" | "mono" | "h1" | "h2" | "h3";

/** All text goes through this so the font, size and color come from the tokens. */
export function Text({ variant = "body", style, ...rest }: TextProps & { variant?: Variant }) {
  const t = useShiso();
  const base: TextStyle = { fontFamily: t.fonts.regular, fontSize: t.d.text, color: t.c.text, lineHeight: Math.round(t.d.text * 1.45) };
  const v: Record<Variant, TextStyle> = {
    body: {},
    muted: { color: t.c.text2 },
    small: { color: t.c.text2, fontSize: t.text.sm, lineHeight: Math.round(t.text.sm * 1.45) },
    label: { fontFamily: t.fonts.medium },
    strong: { fontFamily: t.fonts.semibold },
    mono: { fontFamily: t.fonts.mono, fontSize: t.d.text - 1 },
    h1: { fontFamily: t.fonts.semibold, fontSize: t.text["2xl"], lineHeight: Math.round(t.text["2xl"] * 1.15), letterSpacing: -0.5 },
    h2: { fontFamily: t.fonts.semibold, fontSize: t.text.xl, lineHeight: Math.round(t.text.xl * 1.2), letterSpacing: -0.2 },
    h3: { fontFamily: t.fonts.semibold, fontSize: t.text.lg, lineHeight: Math.round(t.text.lg * 1.3), letterSpacing: -0.2 },
  };
  return <RNText {...rest} style={[base, v[variant], style]} />;
}

/** The one lifted surface. Do not nest cards; use Inset inside. */
export function Card({ children, style }: { children: React.ReactNode; style?: StyleProp<ViewStyle> }) {
  const t = useShiso();
  return (
    <View style={[{ backgroundColor: t.c.card, borderRadius: t.radius.card, padding: t.d.pad, gap: t.d.gap }, t.cardShadow, style]}>
      {children}
    </View>
  );
}

export function Inset({ children, style }: { children: React.ReactNode; style?: StyleProp<ViewStyle> }) {
  const t = useShiso();
  return (
    <View style={[{ backgroundColor: t.c.fill, borderRadius: t.radius.inner, paddingHorizontal: t.space[3], paddingVertical: t.space[1] }, style]}>
      {children}
    </View>
  );
}

export function Row({ children, style }: { children: React.ReactNode; style?: StyleProp<ViewStyle> }) {
  const t = useShiso();
  return <View style={[{ flexDirection: "row", alignItems: "center", flexWrap: "wrap", gap: t.space[2] }, style]}>{children}</View>;
}

type ButtonVariant = "primary" | "secondary" | "outline" | "ghost" | "danger";

/** One primary button per screen. Labels say what happens: "Approve", not "OK". */
export function Button(props: { label: string; onPress?: () => void; variant?: ButtonVariant; disabled?: boolean; grow?: boolean }) {
  const t = useShiso();
  const variant = props.variant ?? "secondary";
  const bg = { primary: t.c.primary, secondary: t.c.fill, outline: "transparent", ghost: "transparent", danger: t.c.badBg }[variant];
  const pressed = { primary: t.c.primaryHover, secondary: t.c.fillHover, outline: t.c.fill, ghost: t.c.fill, danger: t.c.badBg }[variant];
  const fg = { primary: t.c.primaryText, secondary: t.c.text, outline: t.c.text, ghost: t.c.text2, danger: t.c.bad }[variant];
  return (
    <Pressable
      accessibilityRole="button"
      accessibilityState={{ disabled: !!props.disabled }}
      disabled={props.disabled}
      onPress={props.onPress}
      hitSlop={Math.max(0, (44 - t.d.control) / 2)}
      style={(s) => ({
        height: t.d.control,
        paddingHorizontal: Math.round(t.d.control * 0.45),
        borderRadius: t.radius.control,
        borderWidth: 1,
        borderColor: variant === "outline" ? t.c.line : "transparent",
        backgroundColor: s.pressed ? pressed : bg,
        alignItems: "center",
        justifyContent: "center",
        opacity: props.disabled ? 0.5 : 1,
        flexGrow: props.grow ? 1 : 0,
      })}
    >
      <RNText style={{ fontFamily: t.fonts.medium, fontSize: t.d.text, color: fg }}>{props.label}</RNText>
    </Pressable>
  );
}

export type Tone = "neutral" | "ok" | "warn" | "bad" | "soft" | "live";

/** Status: a dot and a word. `live` means an agent is working right now. */
export function Pill({ label, tone = "neutral" }: { label: string; tone?: Tone }) {
  const t = useShiso();
  const m = {
    neutral: [t.c.fill, t.c.text2, undefined],
    ok: [t.c.okBg, t.c.ok, t.c.ok],
    warn: [t.c.warnBg, t.c.warn, t.c.warn],
    bad: [t.c.badBg, t.c.bad, t.c.bad],
    soft: [t.c.soft, t.c.softText, t.c.softText],
    live: [t.c.fill, t.c.text, t.c.live],
  } as const;
  const [bg, fg, dot] = m[tone];
  return (
    <View style={{ flexDirection: "row", alignItems: "center", gap: 6, backgroundColor: bg, borderRadius: t.radius.full, paddingHorizontal: 9, paddingVertical: 2, alignSelf: "flex-start" }}>
      {dot ? <View style={{ width: 6, height: 6, borderRadius: 3, backgroundColor: dot }} /> : null}
      <RNText style={{ fontFamily: t.fonts.medium, fontSize: t.d.text - 2, color: fg }}>{label}</RNText>
    </View>
  );
}

/** Filled text field with its label above. */
export function Input({ label, hint, style, ...rest }: TextInputProps & { label?: string; hint?: string }) {
  const t = useShiso();
  return (
    <View style={{ gap: 6 }}>
      {label ? <Text variant="label">{label}</Text> : null}
      <TextInput
        accessibilityLabel={label}
        placeholderTextColor={t.c.text2}
        {...rest}
        style={[
          {
            minHeight: t.d.control,
            borderRadius: t.radius.control,
            backgroundColor: t.c.fill,
            paddingHorizontal: Math.round(t.d.control * 0.35),
            paddingVertical: rest.multiline ? 10 : 0,
            fontFamily: t.fonts.regular,
            fontSize: t.d.text,
            color: t.c.text,
          },
          style,
        ]}
      />
      {hint ? <Text variant="small">{hint}</Text> : null}
    </View>
  );
}

export function Switch(props: { label: string; value: boolean; onValueChange: (v: boolean) => void }) {
  const t = useShiso();
  return (
    <View style={{ flexDirection: "row", alignItems: "center", justifyContent: "space-between", gap: t.space[3], minHeight: t.d.control }}>
      <Text style={{ flex: 1 }}>{props.label}</Text>
      <RNSwitch
        accessibilityLabel={props.label}
        value={props.value}
        onValueChange={props.onValueChange}
        trackColor={{ false: t.c.line, true: t.c.primary }}
        thumbColor={t.c.card}
        ios_backgroundColor={t.c.line}
      />
    </View>
  );
}

export function Segmented<T extends string>(props: { options: readonly T[]; value: T; onChange: (v: T) => void }) {
  const t = useShiso();
  return (
    <View style={{ flexDirection: "row", gap: 2, padding: 3, backgroundColor: t.c.fill, borderRadius: t.radius.control, alignSelf: "flex-start" }}>
      {props.options.map((o) => {
        const on = o === props.value;
        return (
          <Pressable
            key={o}
            accessibilityRole="tab"
            accessibilityState={{ selected: on }}
            onPress={() => props.onChange(o)}
            style={[
              { height: t.d.control - 8, paddingHorizontal: 12, borderRadius: t.radius.control - 3, justifyContent: "center" },
              on ? { backgroundColor: t.c.card, ...t.cardShadow } : null,
            ]}
          >
            <RNText style={{ fontFamily: t.fonts.medium, fontSize: t.d.text, color: on ? t.c.text : t.c.text2 }}>{o}</RNText>
          </Pressable>
        );
      })}
    </View>
  );
}

/** `fraction` is clamped to 0..1. Always pair with the numbers it stands for. */
export function Meter({ fraction, label }: { fraction: number; label: string }) {
  const t = useShiso();
  const f = Math.min(1, Math.max(0, fraction));
  return (
    <View accessibilityRole="progressbar" accessibilityLabel={label} style={{ height: 8, borderRadius: 4, backgroundColor: t.c.fill, overflow: "hidden" }}>
      <View style={{ width: `${f * 100}%`, height: "100%", borderRadius: 4, backgroundColor: t.c.primary }} />
    </View>
  );
}

/** A label on the left and its value on the right. */
export function KeyValue({ label, value, strong }: { label: string; value: string; strong?: boolean }) {
  return (
    <View style={{ flexDirection: "row", justifyContent: "space-between", gap: 12 }}>
      <Text variant="muted">{label}</Text>
      <Text variant={strong ? "strong" : "body"} style={{ fontVariant: ["tabular-nums"] }}>
        {value}
      </Text>
    </View>
  );
}
