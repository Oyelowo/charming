use echarts::{
    component::{
        Legend, Title, Toolbox, ToolboxFeature, ToolboxFeatureDataView, ToolboxFeatureRestore,
        ToolboxFeatureSaveAsImage, Tooltip,
    },
    df,
    element::{Label, Orient, Position, Sort, TooltipTrigger},
    series::Funnel,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("Funnel").left("left").top("bottom"))
        .tooltip(
            Tooltip::new()
                .trigger(TooltipTrigger::Item)
                .formatter("{a} <br/>{b} : {c}%"),
        )
        .toolbox(
            Toolbox::new()
                .orient(Orient::Vertical)
                .top("center")
                .feature(
                    ToolboxFeature::new()
                        .save_as_image(ToolboxFeatureSaveAsImage::new())
                        .restore(ToolboxFeatureRestore::new())
                        .data_view(ToolboxFeatureDataView::new().read_only(true)),
                ),
        )
        .legend(
            Legend::new()
                .orient(Orient::Vertical)
                .left("left")
                .data(vec!["Show", "Click", "Visit", "Inquiry", "Order"]),
        )
        .series(
            Funnel::new()
                .name("Funnel")
                .width("40%")
                .height("45%")
                .left("5%")
                .top("50%")
                .data(df![
                    (60, "Visit"),
                    (30, "Inquiry"),
                    (10, "Order"),
                    (80, "Click"),
                    (100, "Show")
                ]),
        )
        .series(
            Funnel::new()
                .name("Pyramid")
                .width("40%")
                .height("45%")
                .left("5%")
                .top("5%")
                .sort(Sort::Ascending)
                .data(df![
                    (60, "Visit"),
                    (30, "Inquiry"),
                    (10, "Order"),
                    (80, "Click"),
                    (100, "Show")
                ]),
        )
        .series(
            Funnel::new()
                .name("Funnel")
                .width("40%")
                .height("45%")
                .left("55%")
                .top("5%")
                .label(Label::new().position(Position::Left))
                .data(df![
                    (60, "Visit"),
                    (30, "Inquiry"),
                    (10, "Order"),
                    (80, "Click"),
                    (100, "Show")
                ]),
        )
        .series(
            Funnel::new()
                .name("Pyramid")
                .width("40%")
                .height("45%")
                .left("55%")
                .top("50%")
                .sort(Sort::Ascending)
                .label(Label::new().position(Position::Left))
                .data(df![
                    (60, "Visit"),
                    (30, "Inquiry"),
                    (10, "Order"),
                    (80, "Click"),
                    (100, "Show")
                ]),
        )
}