local libphx = require('libphx').lib
local PayloadConverter = require "Core.Util.PayloadConverter"

function onDef_TaskQueue(t, mt)
    local TaskQueue = t

    -- Wrap worker function in another one with payload FFI GC management.
    ---@param f fun(any): any Payload function
    ---@return (fun(ffi.cdata*): ffi.cdata*)? -- Worker function wrapped in function with FFI Payload data management
    t.MakeWorkerFunction = function(f)
        if type(f) ~= 'function' then
            Log.Error("expected worker function")
            return nil
        end

        return function(payload)
            local managedPayload = Core.ManagedObject(payload, libphx.Payload_Free)
            local result = f(PayloadConverter:payloadToValue(managedPayload))
            return ffi.gc(PayloadConverter:valueToPayload(result, true), nil)
        end
    end
end
