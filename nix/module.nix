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
        ExecStart = "${cfg.package}/bin/evtype_daemon";
        DynamicUser = true;
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
        RestrictNamespaces = true;
        MemoryDenyWriteExecute = true;
        ProtectHostname = true;
        LockPersonality = true;
        ProtectKernelTunables = true;
        RestrictAddressFamilies = ["AF_UNIX"];
        RestrictRealtime = true;
        SystemCallFilter = ["@system-service" "~@resources" "~@privileged"];
        IPAddressDeny = "any";
        RuntimeDirectory = "evtype";
      };
    };
  };
}