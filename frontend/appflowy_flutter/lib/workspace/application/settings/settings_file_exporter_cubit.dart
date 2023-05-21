import 'package:appflowy_backend/protobuf/flowy-folder2/view.pb.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class SettingsFileExportState {
  SettingsFileExportState({
    required this.views,
  }) {
    initialize();
  }

  List<ViewPB> views;
  List<bool> expanded = [];
  List<bool> selectedApps = [];
  List<List<bool>> selectedItems = [];

  SettingsFileExportState copyWith({
    List<ViewPB>? views,
    List<bool>? expanded,
    List<bool>? selectedApps,
    List<List<bool>>? selectedItems,
  }) {
    final state = SettingsFileExportState(
      views: views ?? this.views,
    );
    state.expanded = expanded ?? this.expanded;
    state.selectedApps = selectedApps ?? this.selectedApps;
    state.selectedItems = selectedItems ?? this.selectedItems;
    return state;
  }

  void initialize() {
    expanded = views.map((e) => true).toList();
    selectedApps = views.map((e) => true).toList();
    selectedItems =
        views.map((e) => e.childViews.map((e) => true).toList()).toList();
  }
}

class SettingsFileExporterCubit extends Cubit<SettingsFileExportState> {
  SettingsFileExporterCubit({
    required List<ViewPB> views,
  }) : super(SettingsFileExportState(views: views));

  void selectOrDeselectItem(int outerIndex, int innerIndex) {
    final selectedItems = state.selectedItems;
    selectedItems[outerIndex][innerIndex] =
        !selectedItems[outerIndex][innerIndex];
    emit(state.copyWith(selectedItems: selectedItems));
  }

  void expandOrUnexpandApp(int outerIndex) {
    final expanded = state.expanded;
    expanded[outerIndex] = !expanded[outerIndex];
    emit(state.copyWith(expanded: expanded));
  }

  Map<String, List<String>> fetchSelectedPages() {
    final apps = state.views;
    final selectedItems = state.selectedItems;
    Map<String, List<String>> result = {};
    for (var i = 0; i < selectedItems.length; i++) {
      final selectedItem = selectedItems[i];
      final ids = <String>[];
      for (var j = 0; j < selectedItem.length; j++) {
        if (selectedItem[j]) {
          ids.add(apps[i].childViews[j].id);
        }
      }
      if (ids.isNotEmpty) {
        result[apps[i].id] = ids;
      }
    }
    return result;
  }
}