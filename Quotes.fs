module Quote

open Avalonia.Controls
open Avalonia.FuncUI.DSL
open Avalonia.Layout
open Pages

let view (navigateTo: Page -> unit) =
    Grid.create [
        Grid.rowDefinitions "Auto, *"
        Grid.margin 25
        Grid.children [
            // nav grid
            Grid.create [
                Grid.verticalAlignment VerticalAlignment.Center
                Grid.columnDefinitions "Auto, *, Auto"
                Grid.children [
                    Button.create [
                        Grid.column 0
                        Button.content "Back"
                        Button.onClick (fun _ -> navigateTo Home)
                    ]
                    StackPanel.create [
                        Grid.column 1

                    ]
                    TextBlock.create [
                        Grid.column 2
                        TextBlock.text "Forms"
                        TextBlock.fontSize 24
                    ]
                ]
            ]
            // content grid
            Grid.create [

            ]
        ]
    ]