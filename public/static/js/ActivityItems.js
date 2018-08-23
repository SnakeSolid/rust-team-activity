"use strict";

define([ "ActivityItem", "knockout" ], function(ActivityItem, ko) {
	const ActivityItems = function(items) {
		this.items = items;
	};

	ActivityItems.empty = function() {
		return new ActivityItems([]);
	};

	ActivityItems.fromResponce = function(activity) {
		const items = [];

		for (const author of Object.keys(activity)) {
			const activities = activity[author];
			const item = new ActivityItem(author, activities);

			items.push(item);
		}

		items.sort(function(a, b) {
			return a.author > b.author;
		});

		return new ActivityItems(items);
	};

	return ActivityItems;
});
