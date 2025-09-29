alias catfolder=". printfolder.sh"
mkdir -p ./out/projects

catfolder ../volare_engine_layout > ./out/projects/volare_engine_layout.txt
catfolder ../custom_components > ./out/projects/custom_components.txt
catfolder ../image_renderer > ./out/projects/image_renderer.txt
catfolder ../svg_renderer > ./out/projects/svg_renderer.txt
catfolder ../demo > ./out/projects/demo.txt

catfolder ./out/projects > ./out/source.txt


