open Avalonia
open Avalonia.Controls.ApplicationLifetimes
open Avalonia.Themes.Fluent
open Avalonia.FuncUI.Hosts
open Avalonia.FuncUI.Elmish
open Avalonia.FuncUI.Types
open Elmish

open Pages

type Model = {CurrentPage: Page} 

type Msg =  
    | NavigateTo of Page
    | BackToHome

let init () = {CurrentPage = Home}, Cmd.none

let update msg model =
    match msg with
    | NavigateTo page -> {model with CurrentPage = page}, Cmd.none
    | BackToHome -> { model with CurrentPage = Home }, Cmd.none


let view (model: Model) dispatch =
    match model.CurrentPage with
    | Home -> Home.view (fun page -> dispatch (NavigateTo page)) :> IView
    | Quote -> Quote.view (fun _ -> dispatch BackToHome) :> IView
    | Adjust -> Adjust.view (fun _ -> dispatch BackToHome) :> IView
    | Forms -> Forms.view (fun _ -> dispatch BackToHome) :> IView

type MainWindow() as this =
    inherit HostWindow()
    do
        base.Title <- "Counter Example"
        Program.mkProgram init update view
        |> Program.withHost this
        |> Program.run
        

type App() =
    inherit Application()

    override this.Initialize() =
        this.Styles.Add (FluentTheme())
        this.RequestedThemeVariant <- Styling.ThemeVariant.Default

    override this.OnFrameworkInitializationCompleted() =
        match this.ApplicationLifetime with
        | :? IClassicDesktopStyleApplicationLifetime as desktopLifetime ->
            desktopLifetime.MainWindow <- MainWindow()
        | _ -> ()

module Program =

    [<EntryPoint>]
    let main(args: string[]) =
        AppBuilder
            .Configure<App>()
            .UsePlatformDetect()
            .UseSkia()
            .StartWithClassicDesktopLifetime(args)
