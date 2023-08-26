import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:phosphor_flutter/phosphor_flutter.dart';
import 'package:url_launcher/url_launcher.dart';

class InfoOptionsView extends StatelessWidget {
  const InfoOptionsView({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        ListTile(
          title: Text(AppLocalizations.of(context).license),
          leading: const PhosphorIcon(PhosphorIconsLight.stack),
          onTap: () =>
              launchUrl(Uri.https("go.linwood.dev", "cuboverse/license")),
        ),
        ListTile(
          title: Text(AppLocalizations.of(context).privacyPolicy),
          leading: const PhosphorIcon(PhosphorIconsLight.shield),
          onTap: () => launchUrl(
              Uri.https("docs.cuboverse.linwood.dev", "privacypolicy")),
        ),
        ListTile(
          title: Text(AppLocalizations.of(context).imprint),
          leading: const PhosphorIcon(PhosphorIconsLight.info),
          onTap: () => launchUrl(Uri.https("go.linwood.dev", "imprint")),
        ),
        ListTile(
          title: Text(AppLocalizations.of(context).thirdPartyLicenses),
          leading: const PhosphorIcon(PhosphorIconsLight.file),
          onTap: () => showLicensePage(context: context),
        ),
      ],
    );
  }
}
