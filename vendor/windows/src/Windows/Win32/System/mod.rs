#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_System_AddressBook")]
pub mod AddressBook;
#[cfg(feature = "Win32_System_Antimalware")]
pub mod Antimalware;
#[cfg(feature = "Win32_System_ApplicationInstallationAndServicing")]
pub mod ApplicationInstallationAndServicing;
#[cfg(feature = "Win32_System_ApplicationVerifier")]
pub mod ApplicationVerifier;
#[cfg(feature = "Win32_System_AssessmentTool")]
pub mod AssessmentTool;
#[cfg(feature = "Win32_System_Com")]
pub mod Com;
#[cfg(feature = "Win32_System_ComponentServices")]
pub mod ComponentServices;
#[cfg(feature = "Win32_System_Console")]
pub mod Console;
#[cfg(feature = "Win32_System_Contacts")]
pub mod Contacts;
#[cfg(feature = "Win32_System_DataExchange")]
pub mod DataExchange;
#[cfg(feature = "Win32_System_DeploymentServices")]
pub mod DeploymentServices;
#[cfg(feature = "Win32_System_DesktopSharing")]
pub mod DesktopSharing;
#[cfg(feature = "Win32_System_DeveloperLicensing")]
pub mod DeveloperLicensing;
#[cfg(feature = "Win32_System_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub mod DistributedTransactionCoordinator;
#[cfg(feature = "Win32_System_Environment")]
pub mod Environment;
#[cfg(feature = "Win32_System_ErrorReporting")]
pub mod ErrorReporting;
#[cfg(feature = "Win32_System_EventCollector")]
pub mod EventCollector;
#[cfg(feature = "Win32_System_EventLog")]
pub mod EventLog;
#[cfg(feature = "Win32_System_EventNotificationService")]
pub mod EventNotificationService;
#[cfg(feature = "Win32_System_GroupPolicy")]
pub mod GroupPolicy;
#[cfg(feature = "Win32_System_HostCompute")]
pub mod HostCompute;
#[cfg(feature = "Win32_System_HostComputeNetwork")]
pub mod HostComputeNetwork;
#[cfg(feature = "Win32_System_HostComputeSystem")]
pub mod HostComputeSystem;
#[cfg(feature = "Win32_System_Hypervisor")]
pub mod Hypervisor;
#[cfg(feature = "Win32_System_IO")]
pub mod IO;
#[cfg(feature = "Win32_System_Iis")]
pub mod Iis;
#[cfg(feature = "Win32_System_Ioctl")]
pub mod Ioctl;
#[cfg(feature = "Win32_System_JobObjects")]
pub mod JobObjects;
#[cfg(feature = "Win32_System_Js")]
pub mod Js;
#[cfg(feature = "Win32_System_Kernel")]
pub mod Kernel;
#[cfg(feature = "Win32_System_LibraryLoader")]
pub mod LibraryLoader;
#[cfg(feature = "Win32_System_Mailslots")]
pub mod Mailslots;
#[cfg(feature = "Win32_System_Mapi")]
pub mod Mapi;
#[cfg(feature = "Win32_System_Memory")]
pub mod Memory;
#[cfg(feature = "Win32_System_MessageQueuing")]
pub mod MessageQueuing;
#[cfg(feature = "Win32_System_MixedReality")]
pub mod MixedReality;
#[cfg(feature = "Win32_System_Mmc")]
pub mod Mmc;
#[cfg(feature = "Win32_System_Ole")]
pub mod Ole;
#[cfg(feature = "Win32_System_ParentalControls")]
pub mod ParentalControls;
#[cfg(feature = "Win32_System_PasswordManagement")]
pub mod PasswordManagement;
#[cfg(feature = "Win32_System_Performance")]
pub mod Performance;
#[cfg(feature = "Win32_System_Pipes")]
pub mod Pipes;
#[cfg(feature = "Win32_System_Power")]
pub mod Power;
#[cfg(feature = "Win32_System_ProcessStatus")]
pub mod ProcessStatus;
#[cfg(feature = "Win32_System_PropertiesSystem")]
pub mod PropertiesSystem;
#[cfg(feature = "Win32_System_RealTimeCommunications")]
pub mod RealTimeCommunications;
#[cfg(feature = "Win32_System_Recovery")]
pub mod Recovery;
#[cfg(feature = "Win32_System_Registry")]
pub mod Registry;
#[cfg(feature = "Win32_System_RemoteAssistance")]
pub mod RemoteAssistance;
#[cfg(feature = "Win32_System_RemoteDesktop")]
pub mod RemoteDesktop;
#[cfg(feature = "Win32_System_RemoteManagement")]
pub mod RemoteManagement;
#[cfg(feature = "Win32_System_RestartManager")]
pub mod RestartManager;
#[cfg(feature = "Win32_System_Restore")]
pub mod Restore;
#[cfg(feature = "Win32_System_Rpc")]
pub mod Rpc;
#[cfg(feature = "Win32_System_Search")]
pub mod Search;
#[cfg(feature = "Win32_System_SecurityCenter")]
pub mod SecurityCenter;
#[cfg(feature = "Win32_System_ServerBackup")]
pub mod ServerBackup;
#[cfg(feature = "Win32_System_Services")]
pub mod Services;
#[cfg(feature = "Win32_System_SettingsManagementInfrastructure")]
pub mod SettingsManagementInfrastructure;
#[cfg(feature = "Win32_System_SetupAndMigration")]
pub mod SetupAndMigration;
#[cfg(feature = "Win32_System_Shutdown")]
pub mod Shutdown;
#[cfg(feature = "Win32_System_SideShow")]
pub mod SideShow;
#[cfg(feature = "Win32_System_SqlLite")]
pub mod SqlLite;
#[cfg(feature = "Win32_System_StationsAndDesktops")]
pub mod StationsAndDesktops;
#[cfg(feature = "Win32_System_SubsystemForLinux")]
pub mod SubsystemForLinux;
#[cfg(feature = "Win32_System_SystemInformation")]
pub mod SystemInformation;
#[cfg(feature = "Win32_System_SystemServices")]
pub mod SystemServices;
#[cfg(feature = "Win32_System_TaskScheduler")]
pub mod TaskScheduler;
#[cfg(feature = "Win32_System_Threading")]
pub mod Threading;
#[cfg(feature = "Win32_System_Time")]
pub mod Time;
#[cfg(feature = "Win32_System_TpmBaseServices")]
pub mod TpmBaseServices;
#[cfg(feature = "Win32_System_TransactionServer")]
pub mod TransactionServer;
#[cfg(feature = "Win32_System_UpdateAgent")]
pub mod UpdateAgent;
#[cfg(feature = "Win32_System_UpdateAssessment")]
pub mod UpdateAssessment;
#[cfg(feature = "Win32_System_UserAccessLogging")]
pub mod UserAccessLogging;
#[cfg(feature = "Win32_System_VirtualDosMachines")]
pub mod VirtualDosMachines;
#[cfg(feature = "Win32_System_WinRT")]
pub mod WinRT;
#[cfg(feature = "Win32_System_WindowsProgramming")]
pub mod WindowsProgramming;
#[cfg(feature = "Win32_System_WindowsSync")]
pub mod WindowsSync;
#[cfg(feature = "Win32_System_Wmi")]
pub mod Wmi;