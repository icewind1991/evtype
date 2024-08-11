{
  config,
  lib,
  pkgs,
  ...
}:
with lib; let
  cfg = config.services.evtype;
in {
  options.services.evtype = {
    enable = mkEnableOption "Enables the evtype service";

    package = mkOption {
      type = types.package;
      description = "package to use";
    };
  };

  config = mkIf cfg.enable {
    environment.systemPackages = [cfg.package];

    systemd.services.evtype = {
      wantedBy = ["multi-user.target"];
      description = "EvType daemon";

      serviceConfig = {
        Restart = "on-failure";
        RestartSec = 2;
        ExecStart = "${cfg.package}/bin/evtype_daemon";
        User = "root";
        PrivateTmp = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        NoNewPrivileges = true;
        ProtectClock = true;
        CapabilityBoundingSet = true;
        ProtectKernelLogs = true;
        ProtectControlGroups = true;
        SystemCallArchitectures = "native";
        ProtectKernelModules = true;
        RestrictSUIDSGID = true;
        ProtectProc = "invisible";
        ProcSubset = "pid";
        PrivateUsers = true;
        DevicePolicy = "strict";
        DeviceAllow = ["/dev/uinput" "/dev/stdout"];
        RestrictNamespaces = true;
        MemoryDenyWriteExecute = true;
        ProtectHostname = true;
        LockPersonality = true;
        ProtectKernelTunables = true;
        RestrictAddressFamilies = ["AF_UNIX"];
        RestrictRealtime = true;
        SystemCallFilter = ["@system-service" "~@resources" "~@privileged"];
        PrivateNetwork = true;
        IPAddressDeny = "any";
        RuntimeDirectory = "evtype";
        UMask = "0077";
      };
    };
  };
}