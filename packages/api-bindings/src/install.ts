import {
  InstallAction,
  InstallComponent,
  InstallResponse,
  // eslint-disable-next-line camelcase
  InstallResponse_InstallationStatus,
  NotificationType,
  // eslint-disable-next-line camelcase
  Result_Result,
} from "@amzn/fig-io-proto/fig";

import { sendInstallRequest } from "./requests.js";
import { NotificationResponse, _subscribe } from "./notifications.js";

export type Component =
  | "dotfiles"
  | "ibus"
  | "inputMethod"
  | "accessibility"
  | "desktopEntry"
  | "autostartEntry"
  | "gnomeExtension"
  | "ssh";

function componentToProto(component: Component) {
  switch (component) {
    case "dotfiles":
      return InstallComponent.DOTFILES;
    case "ibus":
      return InstallComponent.IBUS;
    case "accessibility":
      return InstallComponent.ACCESSIBILITY;
    case "inputMethod":
      return InstallComponent.INPUT_METHOD;
    case "ssh":
      return InstallComponent.SSH;
    case "desktopEntry":
      return InstallComponent.DESKTOP_ENTRY;
    case "autostartEntry":
      return InstallComponent.AUTOSTART_ENTRY;
    case "gnomeExtension":
      return InstallComponent.GNOME_EXTENSION;
    default:
      throw Error("Invalid component");
  }
}

function handleBasicResponse(response: InstallResponse) {
  switch (response.response?.$case) {
    case "result":
      // eslint-disable-next-line camelcase
      if (response.response.result.result === Result_Result.OK) {
        return;
      }
      // eslint-disable-next-line camelcase
      if (response.response.result.result === Result_Result.ERROR) {
        throw Error(response.response.result.error);
      } else {
        throw Error(`Unexpected result: ${response.response.result.result}`);
      }
    default:
      throw Error(`Unexpected result: ${response.response?.$case}`);
  }
}

export async function install(component: Component) {
  const response = await sendInstallRequest({
    action: InstallAction.INSTALL,
    component: componentToProto(component),
  });
  handleBasicResponse(response);
}

export async function uninstall(component: Component) {
  const response = await sendInstallRequest({
    action: InstallAction.UNINSTALL,
    component: componentToProto(component),
  });
  handleBasicResponse(response);
}

export async function isInstalled(component: Component) {
  const response = await sendInstallRequest({
    action: InstallAction.STATUS,
    component: componentToProto(component),
  });
  switch (response.response?.$case) {
    case "installationStatus":
      if (
        response.response.installationStatus ===
        // eslint-disable-next-line camelcase
        InstallResponse_InstallationStatus.INSTALLED
      ) {
        return true;
      }
      if (
        response.response.installationStatus ===
        // eslint-disable-next-line camelcase
        InstallResponse_InstallationStatus.NOT_INSTALLED
      ) {
        return false;
      }
      throw Error(`Unexpected result: ${response.response.installationStatus}`);

    default:
      throw Error(`Unexpected result: ${response.response?.$case}`);
  }
}

export const installStatus = {
  subscribe: (
    component: "accessibility",
    handler: (isInstalled: boolean) => NotificationResponse | undefined,
  ) => {
    if (component === "accessibility") {
      return _subscribe(
        { type: NotificationType.NOTIFY_ON_ACCESSIBILITY_CHANGE },
        (notification) => {
          switch (notification?.type?.$case) {
            case "accessibilityChangeNotification":
              return handler(
                notification.type.accessibilityChangeNotification.enabled,
              );
            default:
              break;
          }

          return { unsubscribe: false };
        },
      );
    }
    throw Error("Not implemented");
  },
};