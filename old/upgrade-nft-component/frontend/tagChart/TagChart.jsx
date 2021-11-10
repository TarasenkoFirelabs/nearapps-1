import React, { useState, useEffect } from 'react';
import * as am5 from "@amcharts/amcharts5";
import * as am5wc from "@amcharts/amcharts5/wc";
import am5themes_Animated from "@amcharts/amcharts5/themes/Animated";
import { chartData }  from './data';

export const TagChart = ({callback}) => {
    const [chart, setChart] = useState([]);

    useEffect(() => {
        let root = am5.Root.new("chartdiv");

        root.setThemes([
            am5themes_Animated.new(root)
        ]);

        let series = root.container.children.push(am5wc.WordCloud.new(root, {
            categoryField: "tag",
            valueField: "weight"
          }));

        series.labels.template.events.on("click", (ev) => {
            const tag = ev.target.dataItem.dataContext.tag;

            function onlyUnique(value, index, self) {
              return self.indexOf(value) === index;
            }

            callback(tag);

            // console.log(document.getElementById('tags'));
            // const tags = document.getElementById('tags').value;

            // const newTags = tags === '' ? [] : tags.split(',');
            // newTags.push(tag + 0x13);

            // const uniqueNewTags = newTags.filter(onlyUnique);

            // document.getElementById('tags').value = uniqueNewTags.join(',');
            // const event = new Event('input', { bubbles: true });
            // document.getElementById('tags').dispatchEvent(event);

            // 
        });
          
        series.data.setAll(chartData);

        series.labels.template.setAll({
            fontFamily: "Courier New"
        });
    }, []);

    return (
        <div id="chartdiv" style={{ width: "100%", height: "500px" }}></div>
    )
};
