module Forms

open Avalonia.Controls
open Avalonia.FuncUI.DSL
open Avalonia.Layout
open Pages

let view (navigateTo: Page -> unit) =
        StackPanel.create [
            StackPanel.verticalAlignment VerticalAlignment.Center
            StackPanel.horizontalAlignment HorizontalAlignment.Center
            StackPanel.spacing 20
            StackPanel.children [
                TextBlock.create [
                    TextBlock.text "Forms Page"
                    TextBlock.fontSize 48
                ]
                Button.create [
                    Button.content "Back to Home"
                    Button.onClick (fun _ -> navigateTo Home)
                ]
            ]
        ]
