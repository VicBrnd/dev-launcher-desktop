// src-tauri/src/framework.rs

use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct FrameworkInfo {
    pub name: String,
    pub url: String,
}

const FRAMEWORKS: &[(&str, &str, &str)] = &[
    // ("clé de dépendance", "Nom du framework", "URL du site officiel")
    ("next", "Next.js", "https://nextjs.org/"),
    ("nuxt", "Nuxt.js", "https://nuxtjs.org/"),
    ("gatsby", "Gatsby", "https://www.gatsbyjs.com/"),
    ("remix", "Remix", "https://remix.run/"),
    ("sapper", "Sapper", "https://sapper.svelte.dev/"),
    ("blitz", "Blitz.js", "https://blitzjs.com/"),
    ("gridsome", "Gridsome", "https://gridsome.org/"),
    ("sveltekit", "SvelteKit", "https://kit.svelte.dev/"),
    ("quasar", "Quasar", "https://quasar.dev/"),
    ("ember", "Ember.js", "https://emberjs.com/"),
    ("angular", "Angular", "https://angular.io/"),
    ("vue", "Vue.js", "https://vuejs.org/"),
    ("react-native", "React Native", "https://reactnative.dev/"),
    ("ionic", "Ionic", "https://ionicframework.com/"),
    ("stencil", "Stencil", "https://stenciljs.com/"),
    ("meteor", "Meteor", "https://www.meteor.com/"),
    ("electron", "Electron", "https://www.electronjs.org/"),
    ("expo", "Expo", "https://expo.dev/"),
    ("nativescript", "NativeScript", "https://nativescript.org/"),
    ("tauri", "Tauri", "https://tauri.app/"),
    ("capacitor", "Capacitor", "https://capacitorjs.com/"),
    ("laravel", "Laravel", "https://laravel.com/"),
    // Frameworks généraux
    ("react", "React", "https://reactjs.org/"),
    ("preact", "Preact", "https://preactjs.com/"),
    ("svelte", "Svelte", "https://svelte.dev/"),
    ("solid", "SolidJS", "https://www.solidjs.com/"),
    ("alpine", "Alpine.js", "https://alpinejs.dev/"),
    ("mithril", "Mithril", "https://mithril.js.org/"),
    ("backbone", "Backbone.js", "https://backbonejs.org/"),
    ("aurelia", "Aurelia", "https://aurelia.io/"),
    // Outils de bundling/build
    ("webpack", "Webpack", "https://webpack.js.org/"),
    ("vite", "Vite", "https://vitejs.dev/"),
    ("parcel", "Parcel", "https://parceljs.org/"),
    ("gulp", "Gulp", "https://gulpjs.com/"),
    ("grunt", "Grunt", "https://gruntjs.com/"),
    // Autres frameworks ou outils
    ("express", "Express", "https://expressjs.com/"),
    ("koa", "Koa", "https://koajs.com/"),
    ("hapi", "Hapi", "https://hapi.dev/"),
    ("sails", "Sails.js", "https://sailsjs.com/"),
    ("nest", "NestJS", "https://nestjs.com/"),
    ("adonis", "AdonisJS", "https://adonisjs.com/"),
    ("loopback", "LoopBack", "https://loopback.io/"),
    ("fastify", "Fastify", "https://www.fastify.io/"),
    ("strapi", "Strapi", "https://strapi.io/"),
    ("keystone", "KeystoneJS", "https://keystonejs.com/"),
];

pub fn fetch_framework(path: &PathBuf) -> Option<FrameworkInfo> {
    let package_json_path = path.join("package.json");

    if !package_json_path.exists() {
        return None;
    }

    let file = File::open(&package_json_path).ok()?;
    let reader = BufReader::new(file);
    let package_json: Value = serde_json::from_reader(reader).ok()?;

    let dependencies = package_json.get("dependencies").and_then(|d| d.as_object());
    let dev_dependencies = package_json
        .get("devDependencies")
        .and_then(|d| d.as_object());

    // Fusionner les dépendances et les dépendances de développement
    let mut all_dependencies = serde_json::Map::new();
    if let Some(deps) = dependencies {
        all_dependencies.extend(deps.clone());
    }
    if let Some(dev_deps) = dev_dependencies {
        all_dependencies.extend(dev_deps.clone());
    }

    // Parcourir la liste des frameworks pour trouver une correspondance
    for (key, framework_name, framework_url) in FRAMEWORKS {
        if all_dependencies.contains_key(*key)
            || all_dependencies
                .keys()
                .any(|k| k.starts_with(&format!("@{}/", key)))
            || all_dependencies.contains_key(&format!("@{}", key))
        {
            return Some(FrameworkInfo {
                name: framework_name.to_string(),
                url: framework_url.to_string(),
            });
        }
    }

    None
}