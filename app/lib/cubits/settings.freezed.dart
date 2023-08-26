// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'settings.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$CuboverseSettings {
  String get locale => throw _privateConstructorUsedError;
  ThemeMode get themeMode => throw _privateConstructorUsedError;
  bool get nativeTitleBar => throw _privateConstructorUsedError;
  String get design => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $CuboverseSettingsCopyWith<CuboverseSettings> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CuboverseSettingsCopyWith<$Res> {
  factory $CuboverseSettingsCopyWith(
          CuboverseSettings value, $Res Function(CuboverseSettings) then) =
      _$CuboverseSettingsCopyWithImpl<$Res, CuboverseSettings>;
  @useResult
  $Res call(
      {String locale, ThemeMode themeMode, bool nativeTitleBar, String design});
}

/// @nodoc
class _$CuboverseSettingsCopyWithImpl<$Res, $Val extends CuboverseSettings>
    implements $CuboverseSettingsCopyWith<$Res> {
  _$CuboverseSettingsCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? locale = null,
    Object? themeMode = null,
    Object? nativeTitleBar = null,
    Object? design = null,
  }) {
    return _then(_value.copyWith(
      locale: null == locale
          ? _value.locale
          : locale // ignore: cast_nullable_to_non_nullable
              as String,
      themeMode: null == themeMode
          ? _value.themeMode
          : themeMode // ignore: cast_nullable_to_non_nullable
              as ThemeMode,
      nativeTitleBar: null == nativeTitleBar
          ? _value.nativeTitleBar
          : nativeTitleBar // ignore: cast_nullable_to_non_nullable
              as bool,
      design: null == design
          ? _value.design
          : design // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_CuboverseSettingsCopyWith<$Res>
    implements $CuboverseSettingsCopyWith<$Res> {
  factory _$$_CuboverseSettingsCopyWith(_$_CuboverseSettings value,
          $Res Function(_$_CuboverseSettings) then) =
      __$$_CuboverseSettingsCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {String locale, ThemeMode themeMode, bool nativeTitleBar, String design});
}

/// @nodoc
class __$$_CuboverseSettingsCopyWithImpl<$Res>
    extends _$CuboverseSettingsCopyWithImpl<$Res, _$_CuboverseSettings>
    implements _$$_CuboverseSettingsCopyWith<$Res> {
  __$$_CuboverseSettingsCopyWithImpl(
      _$_CuboverseSettings _value, $Res Function(_$_CuboverseSettings) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? locale = null,
    Object? themeMode = null,
    Object? nativeTitleBar = null,
    Object? design = null,
  }) {
    return _then(_$_CuboverseSettings(
      locale: null == locale
          ? _value.locale
          : locale // ignore: cast_nullable_to_non_nullable
              as String,
      themeMode: null == themeMode
          ? _value.themeMode
          : themeMode // ignore: cast_nullable_to_non_nullable
              as ThemeMode,
      nativeTitleBar: null == nativeTitleBar
          ? _value.nativeTitleBar
          : nativeTitleBar // ignore: cast_nullable_to_non_nullable
              as bool,
      design: null == design
          ? _value.design
          : design // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$_CuboverseSettings extends _CuboverseSettings {
  const _$_CuboverseSettings(
      {this.locale = '',
      this.themeMode = ThemeMode.system,
      this.nativeTitleBar = false,
      this.design = ''})
      : super._();

  @override
  @JsonKey()
  final String locale;
  @override
  @JsonKey()
  final ThemeMode themeMode;
  @override
  @JsonKey()
  final bool nativeTitleBar;
  @override
  @JsonKey()
  final String design;

  @override
  String toString() {
    return 'CuboverseSettings(locale: $locale, themeMode: $themeMode, nativeTitleBar: $nativeTitleBar, design: $design)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_CuboverseSettings &&
            (identical(other.locale, locale) || other.locale == locale) &&
            (identical(other.themeMode, themeMode) ||
                other.themeMode == themeMode) &&
            (identical(other.nativeTitleBar, nativeTitleBar) ||
                other.nativeTitleBar == nativeTitleBar) &&
            (identical(other.design, design) || other.design == design));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, locale, themeMode, nativeTitleBar, design);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_CuboverseSettingsCopyWith<_$_CuboverseSettings> get copyWith =>
      __$$_CuboverseSettingsCopyWithImpl<_$_CuboverseSettings>(
          this, _$identity);
}

abstract class _CuboverseSettings extends CuboverseSettings {
  const factory _CuboverseSettings(
      {final String locale,
      final ThemeMode themeMode,
      final bool nativeTitleBar,
      final String design}) = _$_CuboverseSettings;
  const _CuboverseSettings._() : super._();

  @override
  String get locale;
  @override
  ThemeMode get themeMode;
  @override
  bool get nativeTitleBar;
  @override
  String get design;
  @override
  @JsonKey(ignore: true)
  _$$_CuboverseSettingsCopyWith<_$_CuboverseSettings> get copyWith =>
      throw _privateConstructorUsedError;
}
