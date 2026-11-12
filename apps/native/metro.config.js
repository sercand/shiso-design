// The design system lives outside this app (packages/native, fonts), so Metro has to watch the repo root
// and resolve react/react-native from this app's node_modules for those files.
const { getDefaultConfig } = require("expo/metro-config");
const path = require("path");

const root = path.resolve(__dirname, "../..");
const config = getDefaultConfig(__dirname);
config.watchFolders = [root];
config.resolver.nodeModulesPaths = [path.resolve(__dirname, "node_modules")];
config.resolver.extraNodeModules = { "@shiso/design-native": path.resolve(root, "packages/native") };
module.exports = config;
