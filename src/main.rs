use opentelemetry::global;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_stdout as stdout;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

#[tracing::instrument]
fn task(task_num: usize) {
    function1(task_num);
    function2(task_num);
}

#[tracing::instrument]
fn function1(task: usize) {
    tracing::info!("In function 1, task {}", task);
}
#[tracing::instrument]
fn function2(task: usize) {
    tracing::info!("In function 2, task {}", task);
}

fn main() {
    let provider = TracerProvider::builder()
        .with_simple_exporter(stdout::SpanExporter::default())
        .build();
    let tracer = provider.tracer("test-tracing-instrument");

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let num_tasks = 1;

    eprintln!("Starting {} tasks", num_tasks);

    for task_num in 0..num_tasks {
        task(task_num);
    }

    global::shutdown_tracer_provider();
}
