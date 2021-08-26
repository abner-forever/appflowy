use crate::helper::{spawn_app, TestApp};
use flowy_workspace::entities::{
    app::{App, ColorStyle, CreateAppParams, DeleteAppParams, QueryAppParams, UpdateAppParams},
    view::{CreateViewParams, DeleteViewParams, QueryViewParams, UpdateViewParams, View, ViewType},
    workspace::{
        CreateWorkspaceParams,
        DeleteWorkspaceParams,
        QueryWorkspaceParams,
        UpdateWorkspaceParams,
        Workspace,
    },
};

#[actix_rt::test]
async fn workspace_create() {
    let app = spawn_app().await;
    let (workspace, _) = create_test_workspace(&app).await;
    log::info!("{:?}", workspace);
}

#[actix_rt::test]
async fn workspace_read() {
    let app = spawn_app().await;
    let (workspace_1, _) = create_test_workspace(&app).await;
    let read_params = QueryWorkspaceParams::new(&workspace_1.id);
    log::info!("{:?}", app.read_workspace(read_params).await.unwrap());
}

#[actix_rt::test]
async fn workspace_read_with_belongs() {
    let application = spawn_app().await;
    let (workspace, user_id) = create_test_workspace(&application).await;
    let _ = create_test_app(&application, &workspace.id, &user_id).await;
    let _ = create_test_app(&application, &workspace.id, &user_id).await;
    let _ = create_test_app(&application, &workspace.id, &user_id).await;

    let read_params = QueryWorkspaceParams::new(&workspace.id).read_apps();
    let workspace = application.read_workspace(read_params).await.unwrap();
    assert_eq!(workspace.apps.len(), 3);
}

#[actix_rt::test]
async fn workspace_update() {
    let app = spawn_app().await;
    let (workspace_1, _) = create_test_workspace(&app).await;
    let update_params = UpdateWorkspaceParams {
        id: workspace_1.id.clone(),
        name: Some("workspace 2".to_string()),
        desc: Some("rename workspace description".to_string()),
    };
    app.update_workspace(update_params).await;

    let read_params = QueryWorkspaceParams {
        workspace_id: workspace_1.id.clone(),
        read_apps: false,
    };
    let workspace_2 = app.read_workspace(read_params).await.unwrap();
    log::info!("{:?}", workspace_2);
}

#[actix_rt::test]
async fn workspace_delete() {
    let app = spawn_app().await;
    let (workspace, _) = create_test_workspace(&app).await;
    let delete_params = DeleteWorkspaceParams {
        workspace_id: workspace.id.clone(),
    };

    let _ = app.delete_workspace(delete_params).await;
    let read_params = QueryWorkspaceParams {
        workspace_id: workspace.id.clone(),
        read_apps: false,
    };
    assert_eq!(app.read_workspace(read_params).await.is_none(), true);
}

async fn create_test_workspace(app: &TestApp) -> (Workspace, String) {
    let response = app.register_test_user().await;

    let params = CreateWorkspaceParams {
        name: "My first workspace".to_string(),
        desc: "This is my first workspace".to_string(),
        user_id: response.uid.clone(),
    };
    let workspace = app.create_workspace(params).await;
    (workspace, response.uid)
}

#[actix_rt::test]
async fn app_create() {
    let application = spawn_app().await;
    let (workspace, user_id) = create_test_workspace(&application).await;
    let app = create_test_app(&application, &workspace.id, &user_id).await;
    log::info!("{:?}", app);
}

#[actix_rt::test]
async fn app_read() {
    let application = spawn_app().await;
    let (workspace, user_id) = create_test_workspace(&application).await;
    let app = create_test_app(&application, &workspace.id, &user_id).await;
    let read_params = QueryAppParams::new(&app.id);
    log::info!("{:?}", application.read_app(read_params).await.unwrap());
}

#[actix_rt::test]
async fn app_read_with_belongs() {
    let application = spawn_app().await;
    let (workspace, user_id) = create_test_workspace(&application).await;
    let app = create_test_app(&application, &workspace.id, &user_id).await;

    let _ = create_test_view(&application, &app.id).await;
    let _ = create_test_view(&application, &app.id).await;

    let read_params = QueryAppParams::new(&app.id).read_belongings();
    let app = application.read_app(read_params).await.unwrap();
    assert_eq!(app.belongings.len(), 2);
}

#[actix_rt::test]
async fn app_read_with_belongs_in_trash() {
    let application = spawn_app().await;
    let (workspace, user_id) = create_test_workspace(&application).await;
    let app = create_test_app(&application, &workspace.id, &user_id).await;

    let _ = create_test_view(&application, &app.id).await;
    let view = create_test_view(&application, &app.id).await;

    let update_params = UpdateViewParams::new(&view.id).trash();
    application.update_view(update_params).await;

    let read_params = QueryAppParams::new(&app.id).read_belongings();
    let app = application.read_app(read_params).await.unwrap();
    assert_eq!(app.belongings.len(), 1);
}

#[actix_rt::test]
async fn app_update() {
    let application = spawn_app().await;
    let (workspace, user_id) = create_test_workspace(&application).await;
    let app = create_test_app(&application, &workspace.id, &user_id).await;

    let update_params = UpdateAppParams::new(&app.id).name("flowy");
    application.update_app(update_params).await;

    let read_params = QueryAppParams::new(&app.id);
    let app = application.read_app(read_params).await.unwrap();
    log::info!("{:?}", app);
}

#[actix_rt::test]
async fn app_delete() {
    let application = spawn_app().await;
    let (workspace, user_id) = create_test_workspace(&application).await;
    let app = create_test_app(&application, &workspace.id, &user_id).await;

    let delete_params = DeleteAppParams {
        app_id: app.id.clone(),
    };
    application.delete_app(delete_params).await;

    let read_params = QueryAppParams::new(&app.id);
    assert_eq!(application.read_app(read_params).await.is_none(), true);
}

async fn create_test_app(app: &TestApp, workspace_id: &str, user_id: &str) -> App {
    let params = CreateAppParams {
        workspace_id: workspace_id.to_owned(),
        name: "My first app".to_string(),
        desc: "This is my first app".to_string(),
        color_style: ColorStyle::default(),
        user_id: user_id.to_string(),
    };

    let app = app.create_app(params).await;
    app
}

#[actix_rt::test]
async fn view_create() {
    let application = spawn_app().await;
    let (workspace, user_id) = create_test_workspace(&application).await;
    let app = create_test_app(&application, &workspace.id, &user_id).await;

    let view = create_test_view(&application, &app.id).await;
    log::info!("{:?}", view);
}

#[actix_rt::test]
async fn view_update() {
    let application = spawn_app().await;
    let (workspace, user_id) = create_test_workspace(&application).await;
    let app = create_test_app(&application, &workspace.id, &user_id).await;
    let view = create_test_view(&application, &app.id).await;

    // update
    let update_params = UpdateViewParams::new(&view.id)
        .trash()
        .name("new view name");
    application.update_view(update_params).await;

    // read
    let read_params = QueryViewParams::new(&view.id).trash();
    let view = application.read_view(read_params).await;
    log::info!("{:?}", view);
}

#[actix_rt::test]
async fn view_delete() {
    let application = spawn_app().await;
    let (workspace, user_id) = create_test_workspace(&application).await;
    let app = create_test_app(&application, &workspace.id, &user_id).await;
    let view = create_test_view(&application, &app.id).await;

    // delete
    let delete_params = DeleteViewParams {
        view_id: view.id.clone(),
    };
    application.delete_view(delete_params).await;

    // read
    let read_params = QueryViewParams::new(&view.id).trash();
    assert_eq!(application.read_view(read_params).await.is_none(), true);
}

async fn create_test_view(application: &TestApp, app_id: &str) -> View {
    let params = CreateViewParams {
        belong_to_id: app_id.to_string(),
        name: "My first view".to_string(),
        desc: "This is my first view".to_string(),
        thumbnail: "http://1.png".to_string(),
        view_type: ViewType::Doc,
    };
    let app = application.create_view(params).await;
    app
}

#[actix_rt::test]
async fn workspace_list_read() {
    let application = spawn_app().await;
    let response = application.register_test_user().await;
    for i in 0..3 {
        let params = CreateWorkspaceParams {
            name: format!("{} workspace", i),
            desc: format!("This is my {} workspace", i),
            user_id: response.uid.clone(),
        };
        let _ = application.create_workspace(params).await;
    }

    let workspaces = application.read_workspace_list(&response.uid).await;
    // 3 + 1 (created by default)
    assert_eq!(workspaces.len(), 4);
    log::info!("{:?}", workspaces);
}