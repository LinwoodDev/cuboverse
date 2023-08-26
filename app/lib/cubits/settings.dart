import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:shared_preferences/shared_preferences.dart';

part 'settings.freezed.dart';

@freezed
class CuboverseSettings with _$CuboverseSettings {
  const CuboverseSettings._();

  const factory CuboverseSettings({
    @Default('') String locale,
    @Default(ThemeMode.system) ThemeMode themeMode,
    @Default(false) bool nativeTitleBar,
    @Default('') String design,
  }) = _CuboverseSettings;

  factory CuboverseSettings.fromPrefs(SharedPreferences prefs) =>
      CuboverseSettings(
        themeMode:
            ThemeMode.values.byName(prefs.getString('themeMode') ?? 'system'),
        design: prefs.getString('design') ?? '',
        nativeTitleBar: prefs.getBool('nativeTitleBar') ?? false,
        locale: prefs.getString('locale') ?? '',
      );

  Future<void> save() async {
    final prefs = await SharedPreferences.getInstance();
    prefs.setString('themeMode', themeMode.name);
    prefs.setString('design', design);
    prefs.setBool('nativeTitleBar', nativeTitleBar);
    prefs.setString('locale', locale);
  }
}

class SettingsCubit extends Cubit<CuboverseSettings> {
  SettingsCubit(SharedPreferences prefs)
      : super(CuboverseSettings.fromPrefs(prefs));

  Future<void> setThemeMode(ThemeMode mode) {
    emit(state.copyWith(themeMode: mode));
    return state.save();
  }

  Future<void> setDesign(String design) {
    emit(state.copyWith(design: design));
    return state.save();
  }

  Future<void> setNativeTitleBar(bool nativeTitleBar) {
    emit(state.copyWith(nativeTitleBar: nativeTitleBar));
    return state.save();
  }

  Future<void> setLocale(String locale) {
    emit(state.copyWith(locale: locale));
    return state.save();
  }
}