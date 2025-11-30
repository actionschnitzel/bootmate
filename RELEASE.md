# Release & Publishing Guide

Diese Anleitung beschreibt den kompletten Prozess zum Veröffentlichen einer neuen Version von Boot Mate.

## Übersicht

Bei jedem Release werden automatisch über GitHub Actions gebaut:
- **Snap-Paket** (`bootmate_X.Y.Z_amd64.snap`)
- **DEB-Paket** (`bootmate_X.Y.Z_amd64.deb`)

Beide werden als Release Assets auf GitHub bereitgestellt.

---

## 📋 Release-Prozess (Schritt für Schritt)

### 1. Versionsnummer aktualisieren

Die Version muss in folgenden Dateien angepasst werden:

- `Cargo.toml` (Zeile 3)
- `meson.build` (Zeile 4)
- `snap/snapcraft.yaml` (Zeile 3)
- `po/en.po` (Zeile 8)
- `po/de.po` (Zeile 8)
- `data/ch.srueegger.bootmate.metainfo.xml.in` (Zeile 28 - Version UND Datum)

**Beispiel für Version 1.1.0:**

```bash
# In allen Dateien 1.0.0 → 1.1.0 ersetzen
# Datum in metainfo.xml.in aktualisieren
```

### 2. Release-Notizen in metainfo.xml.in

Aktualisiere die Release-Beschreibung in `data/ch.srueegger.bootmate.metainfo.xml.in`:

```xml
<release version="1.1.0" date="2025-12-15">
  <description>
    <p>Neue Features und Verbesserungen</p>
    <ul>
      <li>Feature 1</li>
      <li>Feature 2</li>
      <li>Bugfix 3</li>
    </ul>
  </description>
</release>
```

### 3. Änderungen committen

```bash
git add .
git commit -m "Bump version to X.Y.Z"
```

### 4. Release-Branch erstellen (optional)

Für größere Releases empfiehlt sich ein Release-Branch:

```bash
git checkout -b release-X.Y.Z
git push -u origin release-X.Y.Z
```

Dann Pull Request erstellen und nach Review in `main` mergen.

### 5. Tag erstellen und pushen

```bash
# Auf main branch wechseln
git checkout main
git pull

# Tag erstellen
git tag -a vX.Y.Z -m "Release version X.Y.Z"

# Tag pushen
git push origin vX.Y.Z
```

### 6. GitHub Release erstellen

1. Gehe zu: https://github.com/srueegger/bootmate/releases
2. Klicke auf **"Draft a new release"**
3. Wähle den Tag: `vX.Y.Z`
4. Release Title: `Boot Mate X.Y.Z`
5. Beschreibung mit Features/Fixes
6. Klicke auf **"Publish release"**

### 7. GitHub Actions läuft automatisch

Nach dem Veröffentlichen des Releases:

1. GitHub Actions startet automatisch (`.github/workflows/release.yml`)
2. Baut Snap-Paket in Ubuntu 24.04 Container
3. Baut DEB-Paket mit `cargo deb`
4. Lädt beide Pakete als Release Assets hoch

**Status überprüfen:**
- Gehe zu **Actions** Tab auf GitHub
- Sieh dir den "Build and Release" Workflow an

**Nach erfolgreichem Build:**
- Die Pakete erscheinen automatisch unter dem Release

---

## 📦 Snap Store veröffentlichen (optional)

### Erstmaliges Setup

```bash
# Bei Snapcraft anmelden
snapcraft login

# App-Name registrieren (nur beim ersten Mal)
snapcraft register bootmate
```

### Snap hochladen

```bash
# Snap von GitHub Release herunterladen
wget https://github.com/srueegger/bootmate/releases/download/vX.Y.Z/bootmate_X.Y.Z_amd64.snap

# Zum Snap Store hochladen
snapcraft upload bootmate_X.Y.Z_amd64.snap
```

**Output:**
```
Revision 1 created for 'bootmate'
```

### Release zu Channel veröffentlichen

```bash
# Zu stable channel releasen
snapcraft release bootmate 1 stable

# ODER zu edge channel für Testing
snapcraft release bootmate 1 edge
```

### Store Grant für Plugs beantragen (nur beim ersten Release)

Die App benötigt folgende Plugs, die eine manuelle Genehmigung brauchen:

- `personal-files` (für `~/.config/autostart`)
- `system-files` (für `/etc/xdg/autostart` und `/usr/share/gnome/autostart`)

**Forum-Post erstellen:**

1. Gehe zu: https://forum.snapcraft.io/c/store-requests/19
2. Erstelle einen neuen Post mit folgendem Template:

```markdown
# Store Grant Request: bootmate

**Snap name:** bootmate
**Store URL:** https://snapcraft.io/bootmate
**Publisher:** Samuel Rüegger

## Requested permissions

### personal-files (dot-config-autostart)
- **Read/Write:** `$HOME/.config/autostart`
- **Reason:** Boot Mate manages user autostart entries by reading and writing .desktop files in the user's autostart directory.

### system-files (system-autostart-read)
- **Read:** `/etc/xdg/autostart` and `/usr/share/gnome/autostart`
- **Reason:** Boot Mate displays system-wide autostart entries from these directories to give users a complete overview.

### system-files (desktop-applications-read)
- **Read:** `/usr/share/applications`
- **Reason:** Boot Mate allows users to add installed applications to autostart by browsing .desktop files.

All permissions are essential for the core functionality of the application.
```

3. Warte auf Approval vom Snapcraft-Team (kann 2-7 Tage dauern)
4. Nach Approval werden die Plugs automatisch connected

---

## 🔧 Troubleshooting

### GitHub Actions Build schlägt fehl

**Snap Build Fehler:**
- Überprüfe `snap/snapcraft.yaml` Syntax
- Stelle sicher, dass alle Dependencies gelistet sind

**DEB Build Fehler:**
- Überprüfe `Cargo.toml` `[package.metadata.deb]` Sektion
- Stelle sicher, dass `build-release` Verzeichnis korrekt ist

### Snap lädt nicht hoch

```bash
# Snapcraft neu anmelden
snapcraft logout
snapcraft login

# Erneut versuchen
snapcraft upload bootmate_X.Y.Z_amd64.snap
```

### Plugs funktionieren nicht

Nach Installation müssen Benutzer die Plugs manuell verbinden:

```bash
sudo snap connect bootmate:dot-config-autostart
sudo snap connect bootmate:system-autostart-read
sudo snap connect bootmate:desktop-applications-read
```

(Nur nötig bis Store Grant genehmigt ist)

---

## 📊 Status überprüfen

### GitHub Release Assets

Gehe zu: https://github.com/srueegger/bootmate/releases/tag/vX.Y.Z

Erwartete Assets:
- ✅ `bootmate_X.Y.Z_amd64.snap`
- ✅ `bootmate_X.Y.Z_amd64.deb`

### Snap Store Status

```bash
# Alle Releases anzeigen
snapcraft status bootmate

# Metrics anzeigen
snapcraft metrics bootmate
```

---

## 🎯 Checkliste für Release

- [ ] Version in allen Dateien aktualisiert
- [ ] Release-Notizen in metainfo.xml.in geschrieben
- [ ] Änderungen committed
- [ ] Tag erstellt und gepusht
- [ ] GitHub Release veröffentlicht
- [ ] GitHub Actions erfolgreich durchgelaufen
- [ ] Release Assets vorhanden (Snap + DEB)
- [ ] (Optional) Snap zum Store hochgeladen
- [ ] (Optional) Snap zu Channel released
- [ ] (Optional) Store Grant beantragt (nur erstes Release)

---

## 📝 Wichtige Notizen

### Automatische Builds

- **Snap**: Wird in Ubuntu 24.04 Container gebaut (`--destructive-mode`)
- **DEB**: Wird mit `cargo deb` gebaut (nutzt Cargo.toml Konfiguration)
- **Trigger**: Automatisch bei Release-Veröffentlichung ODER manuell via "Actions" Tab

### Sandbox Permissions

Die App zeigt ein Banner an, wenn Snap/Flatpak-Plugs nicht verbunden sind:

- **Snap**: Zeigt `sudo snap connect` Befehle
- **Flatpak**: Zeigt `flatpak override` Befehle

### Versionierung

Boot Mate folgt Semantic Versioning:
- **MAJOR.MINOR.PATCH** (z.B. 1.2.3)
- MAJOR: Breaking Changes
- MINOR: Neue Features (backwards compatible)
- PATCH: Bugfixes

---

## 🔗 Wichtige Links

- **GitHub Repository:** https://github.com/srueegger/bootmate
- **GitHub Releases:** https://github.com/srueegger/bootmate/releases
- **Snapcraft Dashboard:** https://snapcraft.io/bootmate
- **Snapcraft Forum:** https://forum.snapcraft.io/c/store-requests/19
- **GitHub Actions:** https://github.com/srueegger/bootmate/actions

---

**Letzte Aktualisierung:** 2025-11-30
**Aktuelle Version:** 1.0.0
