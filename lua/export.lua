local export = {}

local indent = ""

function export.Export(...)
    localised_print({"", indent, ...})
end

function Indent(callback)
    local old_indent = indent
    indent = indent .. "  "
    callback()
    indent = old_indent
end

function export.SetContext(context, callback)
    callback(context)
end

function export.ExportStringAttr(name, value)
    -- Unfortunately we have no control over the string printed by
    -- `localised_print`. There can be single/double quotes or new lines in
    -- there. Neither JSON nor YAML can deal with that well. YAML could if we
    -- had a way to control the indentation, but we don't. So, let's solve it
    -- the hacky way: post-processing from Rust.
    if value ~= nil then
        export.Export(name, ": <STRING>", value, "</STRING>")
    end
end

function export.ExportNumberAttr(name, value)
    -- Number values shouldn't be false, but it happens.
    -- Let's map them to numbers in a sensible way.
    if value ~= nil then
        if value == false then
            value = 0
        end
        if value == true then
            value = 1
        end
        export.Export(name, ": ", value)
    end
end

function export.ExportBoolAttr(name, value)
    if value ~= nil then
        export.Export(name, ": ", value)
    end
end

function export.ExportStringValue(value)
    if value ~= nil then
        export.Export("<STRING>", value, "</STRING>")
    end
end

function export.ExportNumberValue(value)
    if value ~= nil then
        export.Export(value)
    end
end

function export.ExportBoolValue(value)
    if value ~= nil then
        export.Export(value)
    end
end

function export.ExportObject(name, object, callback)
    if object ~= nil then
        export.Export(name, ":")
        Indent(function()
            callback(object)
        end)
    end
end

function export.ExportArray(array, callback)
    if array ~= nil then
        for _, value in ipairs(array) do
            export.Export("- ")
            Indent(function()
                callback(value)
            end)
        end
    end
end

function export.ExportMapping(table, callback)
    if table ~= nil then
        for key, value in pairs(table) do
            export.ExportObject(key, table, function()
                callback(value)
            end)
        end
    end
end

return export
