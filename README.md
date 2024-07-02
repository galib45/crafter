## crafter
```
Description:
  CLI tool to manage android projects

Usage:
  crafter <SUBCOMMAND>
  crafter [OPTIONS]

Subcommands:
  create <APP_NAME> <PACKAGE_NAME>  Create new android project

Options:
  -h, --help           Print help
  -V, --version        Print version
```

### Create new project
```
crafter create "My App" com.example.myapp
```

### Build and install the app
```
cd "My App"
gradle installDebug
```

## Setup the android development environment
This guide was last updated on 20 June 2024
### Setup JDK
- Download JDK from [OpenJDK website](https://openjdk.org/), the latest version as of today is [here](https://jdk.java.net/22/). Extract the archive.
  ```bash
  # Windows
  unzip path\to\the\file.zip
  # Linux/MacOS
  tar zxvf path/to/the/file.tar.gz
  ```
- Now set the `JAVA_HOME` environment variable to the extracted directory.
- Add `JAVA_HOME/bin` to `PATH`.

### Setup Android SDK
- Download the latest "command line tools only" package from the [Android Studio downloads page](https://developer.android.com/studio) and unzip the package.
  ```
  unzip path/to/the/file.zip
  ```
- Move the unzipped `cmdline-tools` directory into a new directory of your choice, such as `android-sdk`. This new directory is your Android SDK directory.
- In the unzipped `cmdline-tools` directory, create a sub-directory called `latest`.
- Move the original `cmdline-tools` directory contents, including the `lib` directory, `bin` directory, `NOTICE.txt` file, and `source.properties` file, into the newly created latest directory. You can now use the command-line tools from this location.
- Now set the `ANDROID_HOME` environment variable to the `android-sdk` directory that contains the `cmdline-tools` folder, and set `ANDROID_USER_HOME` to `ANDROID_HOME/.android`.
- Add `ANDROID_HOME/cmdline-tools/latest/bin` to `PATH`.
- Now download the latest build-tools, platform and platform-tools using the sdkmanager tool.
  ```
  sdkmanager --list | grep -E "build-tools|platform|platform-tools"
  ```
  Choose the version for each of these and install them. You are required to accept the necessary license for each package you have installed. Platform 34 is for Android 14. Platform 35 has been released which is for Android 15 but Android 15 is still in beta.
  ```
  sdkmanager "build-tools;34.0.0" "platforms;android-34" "platform-tools"
  sdkmanager --licenses 
  ```

### Setup Gradle
- Download the latest version of gradle from [their releases page](https://gradle.org/releases/). Unzip it.
  ```
  unzip path/to/the/file.zip
  ```
- Now set the `GRADLE_HOME` environment variable to the extracted directory and set `GRADLE_USER_HOME` to `GRADLE_HOME/.gradle`.
- Add `GRADLE_HOME/bin` to `PATH`.
