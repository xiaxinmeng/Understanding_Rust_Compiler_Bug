plain
2019-11-10T05:34:52.6300872Z 
2019-11-10T05:34:52.6300903Z 
2019-11-10T05:34:52.6300950Z 
2019-11-10T05:34:52.6300982Z 
2019-11-10T05:34:52.6301056Z #![register_attr(my_attr)]
2019-11-10T05:34:52.6301127Z #![register_tool(my_tool)]
2019-11-10T05:34:52.6301211Z #[my_attr] // OK
2019-11-10T05:34:52.6301278Z #[my_tool::anything] // OK
2019-11-10T05:34:52.6301389Z (`rustc_plugin::Registry::register_attribute` is not removed yet, I'll do it in a follow up PR.)
2019-11-10T05:34:52.6301543Z - The naming is `register_attr`/`register_tool` rather than some `register_attributes` (plural, no abbreviation) for consistency with already existing attributes like `cfg_attr`, or `feature`, etc.
2019-11-10T05:34:52.6301753Z ---
2019-11-10T05:34:52.6301807Z ---
2019-11-10T05:34:52.6301874Z ---
2019-11-10T05:34:52.6301932Z AGENT_BUILDDIRECTORY=D:\a\1
---
2019-11-10T05:34:52.6304252Z ANT_HOME=C:\ProgramData\chocolatey\lib\ant\apache-ant-1.10.5
2019-11-10T05:34:52.6304329Z APPDATA=C:\Users\VssAdministrator\AppData\Roaming
2019-11-10T05:34:52.6304424Z AZURE_EXTENSION_DIR=C:\Program Files\Common Files\AzureCliExtensionDirectory
2019-11-10T05:34:52.6305400Z AZURE_HTTP_USER_AGENT=VSTS_d439fc94-e01f-4249-b63e-d8392bc0247c_build_10_0
2019-11-10T05:34:52.6305559Z And remove `#[feature(custom_attribute)]`.
2019-11-10T05:34:52.6305723Z BOOST_ROOT_1_69_0=C:\Program Files\Boost\1.69.0
2019-11-10T05:34:52.6305788Z BUILD_ARTIFACTSTAGINGDIRECTORY=D:\a\1\a
2019-11-10T05:34:52.6305867Z BUILD_BINARIESDIRECTORY=D:\a\1\b
2019-11-10T05:34:52.6305943Z BUILD_BUILDID=13149
---
2019-11-10T05:34:52.6310810Z GeckoWebDriver=C:\SeleniumWebDrivers\GeckoDriver
2019-11-10T05:34:52.6310901Z HOME=/c/Users/VssAdministrator
2019-11-10T05:34:52.6310987Z HOMEDRIVE=C:
2019-11-10T05:34:52.6311050Z HOMEPATH=\Users\VssAdministrator
2019-11-10T05:34:52.6311574Z However, I'd prefer to land *this* PR without an RFC to able to remove `#[feature(custom_attribute)]` and `Registry::register_attribute` while also providing a replacement.
2019-11-10T05:34:52.6311817Z INPUT_ARGUMENTS=
2019-11-10T05:34:52.6311881Z ImageVersion=20191028.1
2019-11-10T05:34:52.6311881Z ImageVersion=20191028.1
2019-11-10T05:34:52.6311981Z It's not clear whether it should go through stabilization or not.
2019-11-10T05:34:52.6312115Z It's quite possible that all the uses should migrate to `#![register_tool]` (https://github.com/rust-lang/rust/issues/66079) instead.
2019-11-10T05:34:52.6312346Z JAVA_HOME_11_X64=C:\Program Files\Java\zulu-11-azure-jdk_11.33.15-11.0.4-win_x64
2019-11-10T05:34:52.6312448Z JAVA_HOME_7_X64=C:\Program Files\Java\zulu-7-azure-jdk_7.31.0.5-7.0.232-win_x64
2019-11-10T05:34:52.6312567Z JAVA_HOME_8_X64=C:\Program Files\Java\zulu-8-azure-jdk_8.40.0.25-8.0.222-win_x64
2019-11-10T05:34:52.6312669Z LOCALAPPDATA=C:\Users\VssAdministrator\AppData\Local
---
2019-11-10T05:34:52.6321560Z SYSTEM_TEAMPROJECTID=e71b0ddf-dd27-435a-873c-e30f86eea377
2019-11-10T05:34:52.6321663Z SYSTEM_TIMELINEID=e35078b8-1d30-49c3-8229-28167d9adf69
2019-11-10T05:34:52.6321737Z SYSTEM_TOTALJOBSINPHASE=16
2019-11-10T05:34:52.6321822Z SYSTEM_WORKFOLDER=D:\a
2019-11-10T05:34:52.6321929Z Some tools (`rustfmt` and `clippy`) used in tool attributes are hardcoded in the compiler.
2019-11-10T05:34:52.6322036Z Support registering inert attributes and attribute tools using crate-level attributes
2019-11-10T05:34:52.6322211Z TEMP=/tmp
2019-11-10T05:34:52.6322283Z TERM=cygwin
2019-11-10T05:34:52.6322343Z TF_BUILD=True
2019-11-10T05:34:52.6322425Z TMP=/tmp
2019-11-10T05:34:52.6322425Z TMP=/tmp
2019-11-10T05:34:52.6322503Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-10T05:34:52.6322598Z TOOLSTATE_PUBLISH=1
2019-11-10T05:34:52.6322703Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-10T05:34:52.6322818Z The previous attempt to introduce them through command line (https://github.com/rust-lang/rust/pull/57921) met some resistance.
2019-11-10T05:34:52.6322945Z This PR introduces a way to do it with a crate level attribute.
2019-11-10T05:34:52.6323155Z This probably needs to go through an RFC before stabilization.
2019-11-10T05:34:52.6323252Z Tracking issues: #66079 (`register_tool`), #66080 (`register_attr`)
2019-11-10T05:34:52.6323403Z USERDOMAIN=fv-az363
2019-11-10T05:34:52.6323475Z USERDOMAIN_ROAMINGPROFILE=fv-az363
2019-11-10T05:34:52.6323536Z USERNAME=VssAdministrator
2019-11-10T05:34:52.6323616Z USERPROFILE=C:\Users\VssAdministrator
2019-11-10T05:34:52.6323616Z USERPROFILE=C:\Users\VssAdministrator
2019-11-10T05:34:52.6323679Z VCPKG_INSTALLATION_ROOT=C:\vcpkg
2019-11-10T05:34:52.6323753Z VCVARS_BAT=vcvars64.bat
2019-11-10T05:34:52.6323830Z VS140COMNTOOLS=C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\Tools\
2019-11-10T05:34:52.6323924Z VSTS_AGENT_PERFLOG=c:\vsts\perflog
2019-11-10T05:34:52.6324012Z VSTS_PROCESS_LOOKUP_ID=vsts_a793f582-7718-41fe-a734-0b3bb3114a89
2019-11-10T05:34:52.6324079Z WINDIR=C:\windows
2019-11-10T05:34:52.6324156Z WIX=C:\Program Files (x86)\WiX Toolset v3.11\
2019-11-10T05:34:52.6324231Z We need some way to introduce them without hardcoding as well.
2019-11-10T05:34:52.6324367Z 