module Forms

open Avalonia.Controls
open Avalonia.FuncUI.DSL
open Avalonia.Layout
open Pages

let view (navigateTo: Page -> unit) =
    Grid.create [
        Grid.rowDefinitions "Auto, *"
        Grid.margin 20
        Grid.children [
            // nav grid
            Grid.create [
                Grid.row 0
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
                Grid.row 1
                Grid.margin 0
                Grid.columnDefinitions "Auto,*"
                Grid.children [
                    TabControl.create [
                        TabControl.tabStripPlacement Dock.Left
                        TabControl.viewItems [
                            TabItem.create [
                                TabItem.header "Sale of a refubished Device"
                                TabItem.content (TextBlock.create [
                                    TextBlock.text "Refurbished Device Sale Form"
                                    TextBlock.margin 5 
                                ])
                            ]
                            TabItem.create [
                                TabItem.header "Sale of a new device"
                                TabItem.content (TextBlock.create [
                                    TextBlock.text "New Device Sale Form"
                                    TextBlock.margin 5
                                ])
                            ]
                            TabItem.create [
                                TabItem.header "Device purchase"
                                TabItem.content (TextBlock.create [
                                    TextBlock.text "Device Purcahse Form"
                                    TextBlock.margin 5
                                ])
                            ]
                            TabItem.create [
                                TabItem.header "Fragile Screen"
                                TabItem.content (TextBlock.create [
                                    TextBlock.text "Fragile Screen Form"
                                    TextBlock.margin 5
                                ])
                            ]
                            TabItem.create [
                                TabItem.header "Back Glass"
                                TabItem.content (TextBlock.create [
                                    TextBlock.text "Back Glass Form"
                                    TextBlock.margin 5
                                ])
                            ]
                        ]
                    ]
                ]
            ]
        ]
    ]