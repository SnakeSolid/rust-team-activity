"use strict";

define([ "ActivityItems", "knockout", "moment", "reqwest" ], function(ActivityItems, ko, moment, reqwest) {
	const HTML_DATE_FORMAT = moment.HTML5_FMT.DATE;
	const USER_DATE_FORMAT = "D MMMM YYYY";

	const Application = function() {
		this.selectedDate = ko.observable(moment().format(HTML_DATE_FORMAT));
		this.activityItems = ko.observable(ActivityItems.empty());
		this.errorMessage = ko.observable("");
		this.isResultVisible = ko.observable(false);
		this.isErrorVisible = ko.observable(false);

		this.isDateValid = ko.pureComputed(function() {
			const selectedDate = this.selectedDate();

			return moment(selectedDate, HTML_DATE_FORMAT, true).isValid();
		}, this);

		this.isDateInvalid = ko.pureComputed(function() {
			return !this.isDateValid();
		}, this);

		this.getErrorMessage = ko.pureComputed(function() {
			return this.errorMessage();
		}, this);
	};

	Application.prototype.showActivity = function() {
		if (!this.isDateValid()) {
			this.errorMessage("Incorrect date");
			this.isErrorVisible(true);

			return;
		}

		this.isResultVisible(false);
		this.isErrorVisible(false);

		const self = this;
		const res = reqwest({
			url: '/api/v1/activity',
			type: 'json',
  			method: 'post',
  			contentType: 'application/json',
  			data: this.getSelectedDate().toString(),
		}).then(function(resp) {
			if (resp.success) {
				self.activityItems(ActivityItems.fromResponce(resp.activity));
				self.isResultVisible(true);
			} else {
				self.errorMessage(msg);
				self.isErrorVisible(true);
			}
		}).fail(function(err, msg) {
			this.errorMessage(msg);
			this.isErrorVisible(true);
		});
	};

	Application.prototype.getResultHeader = function() {
		if (this.isDateValid) {
			const selectedDate = this.selectedDate();

			return "Activity on " + moment(selectedDate, HTML_DATE_FORMAT, true).format(USER_DATE_FORMAT);
		}
	};

	Application.prototype.getSelectedDate = function() {
		const selectedDate = this.selectedDate();

		return moment(selectedDate, HTML_DATE_FORMAT, true).utc().unix();
	};

	return Application;
});
