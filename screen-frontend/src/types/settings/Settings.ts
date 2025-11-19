import { z } from "zod";
import { LayoutType, ColorMode } from "./settings-types";

class Settings {
    layout_type: LayoutType;
    color_mode: ColorMode;

    constructor(layout_type: LayoutType, color_mode: ColorMode) {
        this.layout_type = layout_type;
        this.color_mode = color_mode;
    }
}

const settings_schema = z.object({
    layout_type: z.enum(LayoutType),
    color_mode: z.enum(ColorMode),
});

function parse_settings(json: object): Settings {
    const parse_result = settings_schema.safeParse(json);
    if (parse_result.success) {
        return parse_result.data;
    } else {
        throw new Error("Failed to parse settings: " + parse_result.error.message);
    }
}

export { Settings, parse_settings };
