extern crate cmake;

#[cfg(any(target_os = "linux"))]
extern crate pkg_config;

fn main() {
    // Build the static library with CMake.
    let mut config = cmake::Config::new("rtmidi");
    config.define("BUILD_SHARED_LIBS", "OFF");
    config.define("RTMIDI_BUILD_STATIC_LIBS", "ON");

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=stdc++");

        #[cfg(feature = "alsa")]
        {
            config.define("RTMIDI_API_ALSA", "ON");

            match pkg_config::Config::new().statik(false).probe("alsa") {
                Err(pkg_config::Error::Failure { command, output }) => panic!(
                    "Pkg-config failed - usually this is because alsa development headers are not installed.\n\n\
                    For Fedora users:\n# dnf install alsa-lib-devel\n\n\
                    For Debian/Ubuntu users:\n# apt-get install libasound2-dev\n\n\
                    pkg_config details:\n{}\n", pkg_config::Error::Failure { command, output }),
                Err(e) => panic!("{}", e),
                Ok(alsa_library) => {
                    for lib in alsa_library.libs {
                        println!("cargo:rustc-link-lib={}", lib);
                    }
                }
            };
        }
        #[cfg(not(feature = "alsa"))]
        config.define("RTMIDI_API_ALSA", "OFF");

        #[cfg(feature = "jack_linux")]
        {
            config.define("RTMIDI_API_JACK", "ON");

            match pkg_config::Config::new().statik(false).probe("jack") {
                Err(pkg_config::Error::Failure { command, output }) => panic!(
                    "Pkg-config failed - usually this is because jack development headers are not installed.\n\n\
                    For Debian/Ubuntu users:\n# apt-get install libjack-dev\n\n\
                    pkg_config details:\n{}\n", pkg_config::Error::Failure { command, output }),
                Err(e) => panic!("{}", e),
                Ok(jack_library) => {
                    for lib in jack_library.libs {
                        println!("cargo:rustc-link-lib={}", lib);
                    }
                }
            };
        }
        #[cfg(not(feature = "jack_linux"))]
        config.define("RTMIDI_API_JACK", "OFF");
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=dylib=c++");

        #[cfg(feature = "coremidi")]
        {
            config.define("RTMIDI_API_CORE", "ON");

            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=CoreMidi");
        }
        #[cfg(not(feature = "coremidi"))]
        config.define("RTMIDI_API_CORE", "OFF");

        // TODO: Jack support on MacOS
        // How do you install and link the Jack library files?
        config.define("RTMIDI_API_JACK", "OFF");
        /*
        #[cfg(feature = "jack_macos")]
        config.define("RTMIDI_API_JACK", "ON");
        #[cfg(not(feature = "jack_macos"))]
        config.define("RTMIDI_API_JACK", "OFF");
        */
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=user32");

        #[cfg(feature = "winmm")]
        config.define("RTMIDI_API_WINMM", "ON");
        #[cfg(not(feature = "winmm"))]
        config.define("RTMIDI_API_WINMM", "OFF");

        // https://github.com/rust-lang/rust/issues/39016
        config.profile("Release");
    }

    let dst = config.build();

    // Sometimes the path can be called lib64
    let libdir_path = ["lib", "lib64"]
        .iter()
        .map(|dir| dst.clone().join(dir))
        .find(|path| path.exists())
        .unwrap_or_else(|| {
            panic!(
                "Could not find rtmidi static lib path. Check `target/debug/build/rtmidi-sys-*/out` for a lib or lib64 folder."
            );
        });

    // Tell cargo to link to the compiled library.
    println!(
        "cargo:rustc-link-search=native={}",
        libdir_path.to_str().unwrap()
    );

    println!("cargo:rustc-link-lib=static=rtmidi");
}
