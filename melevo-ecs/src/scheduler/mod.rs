pub use schedule::Schedule;

mod schedule;

enum ScheduleType {
	PreStartup,
	Startup,
	PostStartup,

	PreUpdate,
	Update,
	PostUpdate,
}
