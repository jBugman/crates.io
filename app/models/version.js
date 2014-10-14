import DS from 'ember-data';

export default DS.Model.extend({
    num: DS.attr('string'),
    dl_path: DS.attr('string'),
    created_at: DS.attr('date'),
    updated_at: DS.attr('date'),
    downloads: DS.attr('number'),
    yanked: DS.attr('boolean'),

    crate: DS.belongsTo('crate'),
    authors: DS.hasMany('users', {async: true}),
    dependencies: DS.hasMany('dependency', {async: true}),
    version_downloads: DS.hasMany('version-download', {async: true}),
});
