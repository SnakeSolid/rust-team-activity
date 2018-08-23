"use strict";

define([ "knockout" ], function(ko) {
	const ActivityItem = function(author, activities) {
		this.author = author;
		this.activities = activities;
	};

	return ActivityItem;
});
