module Home 

open Avalonia.Controls
open Avalonia.FuncUI.DSL
open Avalonia.Layout

open Pages

let view (navigateTo: Page -> unit) =
        StackPanel.create [
            StackPanel.orientation Orientation.Vertical
            StackPanel.verticalAlignment VerticalAlignment.Center
            StackPanel.margin 40
            StackPanel.spacing 50
            StackPanel.children [
                TextBlock.create [
                    TextBlock.dock Dock.Top
                    TextBlock.fontSize 72
                    TextBlock.verticalAlignment VerticalAlignment.Center
                    TextBlock.horizontalAlignment HorizontalAlignment.Center
                    TextBlock.text "Casdesk"
                ]
                StackPanel.create [
                    StackPanel.orientation Orientation.Horizontal
                    StackPanel.verticalAlignment VerticalAlignment.Center
                    StackPanel.horizontalAlignment HorizontalAlignment.Center
                    StackPanel.spacing 10
                    StackPanel.children [
                        Button.create [
                            Button.dock Dock.Bottom
                            Button.content "Forms"
                            Button.horizontalAlignment HorizontalAlignment.Stretch
                            Button.horizontalContentAlignment HorizontalAlignment.Center
                            Button.onClick (fun _ -> navigateTo Forms)
                        ]
                        Button.create [
                            Button.dock Dock.Bottom
                            Button.content "Quote"
                            Button.horizontalAlignment HorizontalAlignment.Stretch
                            Button.horizontalContentAlignment HorizontalAlignment.Center
                            Button.onClick (fun _ -> navigateTo Quote)
                        ]
                        Button.create [
                            Button.dock Dock.Bottom
                            Button.content "Adjust POS"
                            Button.horizontalAlignment HorizontalAlignment.Stretch
                            Button.horizontalContentAlignment HorizontalAlignment.Center
                            Button.onClick (fun _ -> navigateTo Adjust)
                        ]
                    ]
                ]                
            ]
        ]