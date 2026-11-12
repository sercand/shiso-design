// Shiso design system demo for React Native. Mirrors apps/html and apps/egui.
import { useFonts } from "expo-font";
import { StatusBar } from "expo-status-bar";
import React, { useState } from "react";
import { Pressable, SafeAreaView, ScrollView, View } from "react-native";
import {
  Button, Card, Input, Inset, KeyValue, Meter, Pill, Row, Segmented, ShisoProvider, Switch, Text,
  fontAssets, useShiso, type Density, type Mode, type Tone,
} from "@shiso/design-native";

const TICKETS: { key: string; title: string; who: string; status: string; tone: Tone; cost: string }[] = [
  { key: "OPS-142", title: "Reconcile October provider invoices", who: "Mira", status: "In review", tone: "warn", cost: "$3.84" },
  { key: "ENG-88", title: "Fix crash on save in the level editor", who: "Kai", status: "Coding", tone: "soft", cost: "$11.02" },
  { key: "LEG-12", title: "Draft NDA for Northwind Audio", who: "Lexi", status: "Validator failed", tone: "bad", cost: "$0.61" },
  { key: "OPS-139", title: "Roster for week 47", who: "Deniz", status: "Done", tone: "ok", cost: "$0.00" },
];
const TABS = ["Home", "Approvals", "Tickets", "Copilot"] as const;

export default function App() {
  const [loaded] = useFonts(fontAssets);
  const [mode, setMode] = useState<Mode | "system">("system");
  const [density, setDensity] = useState<Density>("comfortable");
  if (!loaded) return null;
  return (
    <ShisoProvider mode={mode} density={density}>
      <Screen mode={mode} setMode={setMode} density={density} setDensity={setDensity} />
    </ShisoProvider>
  );
}

function Screen(props: { mode: Mode | "system"; setMode: (m: Mode | "system") => void; density: Density; setDensity: (d: Density) => void }) {
  const t = useShiso();
  const [section, setSection] = useState<"Changes" | "Tool calls">("Changes");
  const [tab, setTab] = useState<(typeof TABS)[number]>("Approvals");
  const [budget, setBudget] = useState("$60.00");
  const [note, setNote] = useState("");
  const [review, setReview] = useState(true);
  const diff: [string, string, string][] = [["Senior artist", "₺9.500", "₺10.400"], ["QA tester", "₺5.200", "₺5.600"], ["Effective from", "", "1 Jan 2027"]];

  return (
    <SafeAreaView style={{ flex: 1, backgroundColor: t.c.canvas }}>
      <StatusBar style={t.mode === "dark" ? "light" : "dark"} />
      <ScrollView contentContainerStyle={{ padding: t.space[4], gap: t.d.gap, maxWidth: 560, width: "100%", alignSelf: "center" }}>
        <Text variant="h1">Approvals</Text>
        <Segmented options={["Changes", "Tool calls"] as const} value={section} onChange={setSection} />

        <Card>
          <Row>
            <Pill label="In review" tone="warn" />
            <Pill label="2 of 3 parts approved" />
          </Row>
          <View>
            <Text variant="h3">Raise contractor day rates for 2027</Text>
            <Text variant="muted">Proposed by Mira, finance agent. Applies when approved.</Text>
          </View>
          <Inset>
            {diff.map(([field, before, after], i) => (
              <View key={field} style={{ flexDirection: "row", justifyContent: "space-between", paddingVertical: 7, borderTopWidth: i ? 1 : 0, borderTopColor: t.c.line }}>
                <Text>{field}</Text>
                <Row>
                  {before ? <Text variant="muted" style={{ textDecorationLine: "line-through" }}>{before}</Text> : null}
                  <Text variant="strong">{after}</Text>
                </Row>
              </View>
            ))}
          </Inset>
          <Row>
            <Button label="Approve" variant="primary" grow />
            <Button label="Request changes" />
          </Row>
        </Card>

        <Card>
          <Row style={{ flexWrap: "nowrap" }}>
            <View style={{ flex: 1 }}>
              <Text variant="h3">Mira</Text>
              <Text variant="muted">Finance agent, owned by Deniz</Text>
            </View>
            <Pill label="Running" tone="live" />
          </Row>
          <View style={{ gap: 6 }}>
            <KeyValue label="Budget this month" value="$41.20 of $60.00" strong />
            <Meter fraction={41.2 / 60} label="69% of budget used" />
          </View>
          <KeyValue label="Model tier" value="Local and mid" />
          <KeyValue label="Tickets accepted" value="18" />
        </Card>

        <Card>
          <Text variant="h3">Agent policy</Text>
          <Input label="Monthly budget" value={budget} onChangeText={setBudget} hint="The agent pauses when it reaches this amount." />
          <Switch label="Review before marking done" value={review} onValueChange={setReview} />
          <Input label="Note for the owner" value={note} onChangeText={setNote} placeholder="Why this budget?" multiline />
          <Row>
            <Button label="Save changes" variant="primary" />
            <Button label="Cancel" variant="outline" />
            <Button label="Offboard" variant="danger" />
          </Row>
        </Card>

        <Card>
          <Text variant="h3">Open tickets</Text>
          {TICKETS.map((k, i) => (
            <View key={k.key} style={{ gap: 4, paddingTop: i ? t.d.gap : 0, borderTopWidth: i ? 1 : 0, borderTopColor: t.c.line }}>
              <Text variant="label">{k.title}</Text>
              <Row>
                <Text variant="mono" style={{ color: t.c.text2 }}>{k.key}</Text>
                <Text variant="muted">{k.who}</Text>
                <Pill label={k.status} tone={k.tone} />
                <View style={{ flex: 1 }} />
                <Text style={{ fontVariant: ["tabular-nums"] }}>{k.cost}</Text>
              </Row>
            </View>
          ))}
        </Card>

        <Card>
          <Text variant="h3">Type</Text>
          <Text variant="h1">Every agent has an owner</Text>
          <Text variant="h2">Değişiklik isteği İstanbul şubesinde onay bekliyor</Text>
          <Text>Body text at the base size. <Text variant="muted">Secondary text sits one step back.</Text></Text>
          <Text variant="mono">open_change_request → cr_8f3a2c</Text>
        </Card>

        <Card>
          <Text variant="h3">Appearance</Text>
          <Segmented options={["system", "light", "dark"] as const} value={props.mode} onChange={props.setMode} />
          <Segmented options={["comfortable", "compact"] as const} value={props.density} onChange={props.setDensity} />
        </Card>
      </ScrollView>

      <View style={{ flexDirection: "row", backgroundColor: t.c.card, borderTopWidth: 1, borderTopColor: t.c.line, paddingTop: 8, paddingBottom: 10 }}>
        {TABS.map((name) => (
          <Pressable key={name} accessibilityRole="tab" accessibilityState={{ selected: tab === name }} onPress={() => setTab(name)} style={{ flex: 1, alignItems: "center", minHeight: 44, justifyContent: "center" }}>
            <Text variant="label" style={{ fontSize: t.text.sm, color: tab === name ? t.c.primary : t.c.text2 }}>{name}</Text>
          </Pressable>
        ))}
      </View>
    </SafeAreaView>
  );
}
