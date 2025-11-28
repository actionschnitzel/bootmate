desktop_file = i18n.merge_file(
  input: '@0@.desktop.in'.format(base_id),
  output: '@0@.desktop'.format(base_id),
  type: 'desktop',
  po_dir: '../po',
  install: true,
  install_dir: applicationsdir,
)

if desktop_file_validate.found()
  test('Validate desktop file', desktop_file_validate,
    args: [desktop_file]
  )
endif

metainfo_file = i18n.merge_file(
  input: '@0@.metainfo.xml.in'.format(base_id),
  output: '@0@.metainfo.xml'.format(base_id),
  po_dir: '../po',
  install: true,
  install_dir: metainfodir,
)

if appstream_util.found()
  test('Validate metainfo file', appstream_util,
    args: ['validate', '--nonet', metainfo_file]
  )
endif

# GSchema will be added later
# install_data('@0@.gschema.xml'.format(base_id),
#   install_dir: datadir / 'glib-2.0/schemas'
# )

subdir('icons')

# Compile resources
gnome.compile_resources(
  'bootmate',
  'bootmate.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)
