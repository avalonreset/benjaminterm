#![cfg(windows)]

use crate::ToastNotification as TN;
use xml::escape::escape_str_pcdata;

use windows::core::{Error as WinError, IInspectable, Interface, HSTRING};
use windows::Data::Xml::Dom::XmlDocument;
use windows::Foundation::TypedEventHandler;
use windows::Win32::Foundation::E_POINTER;
use windows::UI::Notifications::{
    ToastActivatedEventArgs, ToastNotification, ToastNotificationManager,
};

fn unwrap_arg<T>(a: &Option<T>) -> Result<&T, WinError> {
    match a {
        Some(t) => Ok(t),
        None => Err(WinError::new(E_POINTER, HSTRING::from("option is none"))),
    }
}

fn escape_str_attribute(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn show_notif_impl(toast: TN) -> Result<(), Box<dyn std::error::Error>> {
    let xml = XmlDocument::new()?;

    let launch_argument = toast
        .click_arguments
        .as_deref()
        .or_else(|| toast.url.as_deref().map(|_| "show"));
    let launch_attr = launch_argument
        .map(|arg| format!(r#" launch="{}""#, escape_str_attribute(arg)))
        .unwrap_or_default();
    let scenario_attr = toast
        .scenario
        .as_deref()
        .map(|scenario| format!(r#" scenario="{}""#, escape_str_attribute(scenario)))
        .unwrap_or_default();

    let url_actions = if toast.url.is_some() {
        r#"
        <actions>
           <action content="Show" arguments="show" />
        </actions>
        "#
        .to_string()
    } else if let Some(arguments) = toast.click_arguments.as_deref() {
        format!(
            r#"
        <actions>
           <action content="Focus" arguments="{}" activationType="foreground" />
        </actions>
        "#,
            escape_str_attribute(arguments)
        )
    } else {
        String::new()
    };

    xml.LoadXml(HSTRING::from(format!(
        r#"<toast{}{} duration="long">
        <visual>
            <binding template="ToastGeneric">
                <text>{}</text>
                <text>{}</text>
            </binding>
        </visual>
        <audio silent="true"/>
        {}
    </toast>"#,
        launch_attr,
        scenario_attr,
        escape_str_pcdata(&toast.title),
        escape_str_pcdata(&toast.message),
        url_actions
    )))?;

    let notif = ToastNotification::CreateToastNotification(xml)?;
    if let Some(tag) = toast.tag.as_deref() {
        notif.SetTag(&HSTRING::from(tag))?;
    }
    if let Some(group) = toast.group.as_deref() {
        notif.SetGroup(&HSTRING::from(group))?;
    }

    notif.Activated(TypedEventHandler::new(
        move |_: &Option<ToastNotification>, result: &Option<IInspectable>| {
            // let myself = unwrap_arg(myself)?;
            let result = unwrap_arg(result)?.cast::<ToastActivatedEventArgs>()?;

            let args = result.Arguments()?.to_string();

            if args == "show" {
                if let Some(url) = toast.url.as_ref() {
                    wezterm_open_url::open_url(url);
                }
            }
            crate::dispatch_toast_activation(args);

            Ok(())
        },
    ))?;

    /*
    notif.dismissed(TypedEventHandler::new(|sender, result| {
        log::info!("dismissed {:?}", result);
        Ok(())
    }))?;

    notif.failed(TypedEventHandler::new(|sender, result| {
        log::warn!("toasts are disabled {:?}", result);
        Ok(())
    }))?;
    */

    let notifier = ToastNotificationManager::CreateToastNotifierWithId(HSTRING::from(
        "com.avalonreset.benjaminterm",
    ))?;

    notifier.Show(&notif)?;

    Ok(())
}

fn dismiss_toast_notification_impl(
    tag: &str,
    group: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let history = ToastNotificationManager::History()?;
    let tag = HSTRING::from(tag);

    if let Some(group) = group {
        let group = HSTRING::from(group);
        let app_id = HSTRING::from("com.avalonreset.benjaminterm");
        history.RemoveGroupedTagWithId(&tag, &group, &app_id)?;
    } else {
        history.Remove(&tag)?;
    }

    Ok(())
}

fn dismiss_toast_notification_group_impl(group: &str) -> Result<(), Box<dyn std::error::Error>> {
    let history = ToastNotificationManager::History()?;
    let group = HSTRING::from(group);
    let app_id = HSTRING::from("com.avalonreset.benjaminterm");
    history.RemoveGroupWithId(&group, &app_id)?;

    Ok(())
}

pub fn dismiss_toast_notification(
    tag: &str,
    group: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let tag = tag.to_string();
    let group = group.map(str::to_string);

    std::thread::spawn(move || {
        if let Err(err) = dismiss_toast_notification_impl(&tag, group.as_deref()) {
            log::error!("Failed to dismiss toast notification: {:#}", err);
        }
    });

    Ok(())
}

pub fn dismiss_toast_notification_group(group: &str) -> Result<(), Box<dyn std::error::Error>> {
    let group = group.to_string();

    std::thread::spawn(move || {
        if let Err(err) = dismiss_toast_notification_group_impl(&group) {
            log::error!("Failed to dismiss toast notification group: {:#}", err);
        }
    });

    Ok(())
}

pub fn show_notif(notif: TN) -> Result<(), Box<dyn std::error::Error>> {
    // We need to be in a different thread from the caller
    // in case we get called in the guts of a windows message
    // loop dispatch and are unable to pump messages
    std::thread::spawn(move || {
        if let Err(err) = show_notif_impl(notif) {
            log::error!("Failed to show toast notification: {:#}", err);
        }
    });

    Ok(())
}
