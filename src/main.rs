use bevy::prelude::{
    App, Commands, Component, Plugin, Query, Res, ResMut, Resource, Time, Timer, With,
};
use bevy::time::TimerMode;
use bevy::DefaultPlugins;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// リソースとなるデータは Resource を派生させる v0.8.0 -> v0.9.0
#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resource (グローバルなデータ)を追加
            //   mode 指定がわかりやすくなった
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            // System をアプリケーションのスタートアップスケジュールに追加
            //   event loop の外で最初に実行される
            .add_startup_system(add_people)
            // System をアプリケーションのスケジュールに追加
            //   event loop に登録される
            .add_system(greet_people);
    }
}

// System
fn add_people(mut commands: Commands) {
    // spawn() は廃止されたみたい。デフォルト設定(bundle)する必要有り。 v0.8.0 -> v0.9.0
    // 以前のようにやるには spawn_empty() を使う
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayba Nieves".to_string())));
}

// System
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // time.delta():
    //   アプリ実行からの経過時間を秒で取得
    // timer.0.tick(time.delta()):
    //   timer を time.delta() だけ進める
    // timer.0.tick(time.delta()).just_finished():
    //   timer を進めて duration に達したとき true を返す
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}", name.0);
        }
    }
}

// メイン
fn main() {
    // アプリケーションインスタンス作成
    App::new()
        /*
            DefaultPlugins は下記と同等
            .add_plugin(CorePlugin::default())
            .add_plugin(InputPlugin::default())
            .add_plugin(WindowPlugin::default())
        */
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        // アプリケーション実行
        //   ※スケジュールされた System は可能な限り並列実行する(デフォルト)
        .run();
}
