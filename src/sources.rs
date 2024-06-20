pub struct Sources {
	pub settings_gradle_kts: String,
	pub gradle_properties: String,
	pub root_gitignore: String,
	pub app_gitignore: String,
	pub root_build_gradle_kts: String,
	pub app_build_gradle_kts: String,
	pub libs_versions_toml: String,
	pub android_manifest_xml: String,
	pub resources_xml: String,
	pub main_activity_kt: String,
	pub color_kt: String,
	pub theme_kt: String,
	pub type_kt: String
}

impl Sources {
	pub fn generate(app_name: &str, package_name: &str) -> Self {
        let app_name_nospace = app_name.replace(" ", "");
        let settings_gradle_kts = format!(r#"pluginManagement {{
    repositories {{
        google {{
            content {{
                includeGroupByRegex("com\\.android.*")
                includeGroupByRegex("com\\.google.*")
                includeGroupByRegex("androidx.*")
            }}
        }}
        mavenCentral()
        gradlePluginPortal()
    }}
}}
dependencyResolutionManagement {{
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {{
        google()
        mavenCentral()
    }}
}}

rootProject.name = "{app_name}"
include(":app")"#);

		let gradle_properties = r#"# Project-wide Gradle settings
# Increase the heap size
org.gradle.jvmargs=-Xmx2048m -Dfile.encoding=UTF-8

# Enable the configuration cache
org.gradle.configuration-cache=true

# Enable the build cache
org.gradle.caching=true

# AndroidX package structure to make it clearer which packages are bundled with the
# Android operating system, and which are packaged with your app's APK
# https://developer.android.com/topic/libraries/support-library/androidx-rn
android.useAndroidX=true

# Kotlin code style for this project: "official" or "obsolete":
kotlin.code.style=official

# Enables namespacing of each library's R class so that its R class includes only the
# resources declared in the library itself and none from the library's dependencies,
# thereby reducing the size of the R class for that library
android.nonTransitiveRClass=true"#.to_string();

		let root_gitignore = ".gradle\nbuild".to_string();
		let app_gitignore = "build".to_string();

		let root_build_gradle_kts = r"// Top-level build file where you can add configuration options common to all sub-projects/modules.
plugins {
    alias(libs.plugins.android.application) apply false
    alias(libs.plugins.jetbrains.kotlin.android) apply false
}".to_string();

		let app_build_gradle_kts = format!(r#"plugins {{
    alias(libs.plugins.android.application)
    alias(libs.plugins.jetbrains.kotlin.android)
}}

android {{
    namespace = "{package_name}"
    compileSdk = 34

    defaultConfig {{
        applicationId = "{package_name}"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"

        vectorDrawables {{
            useSupportLibrary = true
        }}
    }}

    buildTypes {{
        release {{
            isMinifyEnabled = false
        }}
    }}
    compileOptions {{
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }}
    kotlinOptions {{
        jvmTarget = "1.8"
    }}
    buildFeatures {{
        compose = true
    }}
    composeOptions {{
        kotlinCompilerExtensionVersion = "1.5.1"
    }}
    packaging {{
        resources {{
            excludes += "/META-INF/{{AL2.0,LGPL2.1}}"
        }}
    }}
}}

dependencies {{
    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.lifecycle.runtime.ktx)
    implementation(libs.androidx.activity.compose)
    implementation(platform(libs.androidx.compose.bom))
    implementation(libs.androidx.ui)
    implementation(libs.androidx.ui.graphics)
    implementation(libs.androidx.material3)
}}"#);

		let libs_versions_toml = r#"[versions]
agp = "8.5.0"
kotlin = "1.9.0"
coreKtx = "1.10.1"
lifecycleRuntimeKtx = "2.6.1"
activityCompose = "1.8.0"
composeBom = "2024.04.01"

[libraries]
androidx-core-ktx = { group = "androidx.core", name = "core-ktx", version.ref = "coreKtx" }
androidx-lifecycle-runtime-ktx = { group = "androidx.lifecycle", name = "lifecycle-runtime-ktx", version.ref = "lifecycleRuntimeKtx" }
androidx-activity-compose = { group = "androidx.activity", name = "activity-compose", version.ref = "activityCompose" }
androidx-compose-bom = { group = "androidx.compose", name = "compose-bom", version.ref = "composeBom" }
androidx-ui = { group = "androidx.compose.ui", name = "ui" }
androidx-ui-graphics = { group = "androidx.compose.ui", name = "ui-graphics" }
androidx-material3 = { group = "androidx.compose.material3", name = "material3" }

[plugins]
android-application = { id = "com.android.application", version.ref = "agp" }
jetbrains-kotlin-android = { id = "org.jetbrains.kotlin.android", version.ref = "kotlin" }
"#.to_string();

		let android_manifest_xml = format!(r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    xmlns:tools="http://schemas.android.com/tools">

    <application
        android:label="@string/app_name"
        android:supportsRtl="true"
        android:theme="@style/Theme.{app_name_nospace}"
        tools:targetApi="31">
        <activity
            android:name=".MainActivity"
            android:exported="true"
            android:label="@string/app_name">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>

</manifest>"#);

		let resources_xml = format!(r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="app_name">{app_name}</string>
    <style name="Theme.{app_name_nospace}" parent="android:Theme.Material.Light.NoActionBar" />
</resources>"#);

		let main_activity_kt = format!(r#"package {package_name}

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import {package_name}.ui.theme.{app_name_nospace}Theme

class MainActivity : ComponentActivity() {{
    override fun onCreate(savedInstanceState: Bundle?) {{
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {{
            {app_name_nospace}Theme {{
                Scaffold(modifier = Modifier.fillMaxSize()) {{ innerPadding ->
                    Greeting(
                        name = "Android",
                        modifier = Modifier.padding(innerPadding)
                    )
                }}
            }}
        }}
    }}
}}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {{
    Text(
        text = "Hello $name from crafter!",
        modifier = modifier
    )
}}"#);

		let color_kt = format!(r#"package {package_name}.ui.theme

import androidx.compose.ui.graphics.Color

val Purple80 = Color(0xFFD0BCFF)
val PurpleGrey80 = Color(0xFFCCC2DC)
val Pink80 = Color(0xFFEFB8C8)

val Purple40 = Color(0xFF6650a4)
val PurpleGrey40 = Color(0xFF625b71)
val Pink40 = Color(0xFF7D5260)"#);

		let theme_kt = format!(r#"package {package_name}.ui.theme

import android.app.Activity
import android.os.Build
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.dynamicDarkColorScheme
import androidx.compose.material3.dynamicLightColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalContext

private val DarkColorScheme = darkColorScheme(
    primary = Purple80,
    secondary = PurpleGrey80,
    tertiary = Pink80
)

private val LightColorScheme = lightColorScheme(
    primary = Purple40,
    secondary = PurpleGrey40,
    tertiary = Pink40

    /* Other default colors to override
    background = Color(0xFFFFFBFE),
    surface = Color(0xFFFFFBFE),
    onPrimary = Color.White,
    onSecondary = Color.White,
    onTertiary = Color.White,
    onBackground = Color(0xFF1C1B1F),
    onSurface = Color(0xFF1C1B1F),
    */
)

@Composable
fun {app_name_nospace}Theme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    // Dynamic color is available on Android 12+
    dynamicColor: Boolean = true,
    content: @Composable () -> Unit
) {{
    val colorScheme = when {{
        dynamicColor && Build.VERSION.SDK_INT >= Build.VERSION_CODES.S -> {{
            val context = LocalContext.current
            if (darkTheme) dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
        }}

        darkTheme -> DarkColorScheme
        else -> LightColorScheme
    }}

    MaterialTheme(
        colorScheme = colorScheme,
        typography = Typography,
        content = content
    )
}}"#);

		let type_kt = format!(r#"package {package_name}.ui.theme

import androidx.compose.material3.Typography
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.sp

// Set of Material typography styles to start with
val Typography = Typography(
    bodyLarge = TextStyle(
        fontFamily = FontFamily.Default,
        fontWeight = FontWeight.Normal,
        fontSize = 16.sp,
        lineHeight = 24.sp,
        letterSpacing = 0.5.sp
    )
    /* Other default text styles to override
    titleLarge = TextStyle(
        fontFamily = FontFamily.Default,
        fontWeight = FontWeight.Normal,
        fontSize = 22.sp,
        lineHeight = 28.sp,
        letterSpacing = 0.sp
    ),
    labelSmall = TextStyle(
        fontFamily = FontFamily.Default,
        fontWeight = FontWeight.Medium,
        fontSize = 11.sp,
        lineHeight = 16.sp,
        letterSpacing = 0.5.sp
    )
    */
)"#);

		Self {
			settings_gradle_kts,
			gradle_properties,
			root_gitignore,
			app_gitignore,
			root_build_gradle_kts,
			app_build_gradle_kts,
			libs_versions_toml,
			android_manifest_xml,
			resources_xml,
			main_activity_kt,
			color_kt,
			theme_kt,
			type_kt
		}
	}
}