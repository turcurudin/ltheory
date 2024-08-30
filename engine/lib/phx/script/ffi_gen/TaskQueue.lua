-- TaskQueue -------------------------------------------------------------------
local Loader = {}

function Loader.declareType()
    ffi.cdef [[
        typedef struct TaskQueue {} TaskQueue;
    ]]

    return 1, 'TaskQueue'
end

function Loader.defineType()
    local ffi = require('ffi')
    local libphx = require('libphx').lib
    local TaskQueue

    do -- C Definitions
        ffi.cdef [[
            void          TaskQueue_Free             (TaskQueue*);
            bool          TaskQueue_StartWorker      (TaskQueue*, uint8 workerId, cstr workerName, cstr scriptPath);
            bool          TaskQueue_StopWorker       (TaskQueue const*, uint8 workerId);
            bool          TaskQueue_IsWorkerFinished (TaskQueue const*, uint8 workerId);
            void          TaskQueue_StopAllWorkers   (TaskQueue const*);
            uint64 const* TaskQueue_TasksInProgress  (TaskQueue const*, uint8 workerId);
            uint64 const* TaskQueue_SendTask         (TaskQueue*, uint8 workerId, Payload* data);
            TaskResult*   TaskQueue_NextTaskResult   (TaskQueue*, uint8 workerId);
            bool          TaskQueue_SendEcho         (TaskQueue*, cstr data);
            cstr          TaskQueue_GetEcho          (TaskQueue*);
        ]]
    end

    do -- Global Symbol Table
        TaskQueue = {}

        if onDef_TaskQueue then onDef_TaskQueue(TaskQueue, mt) end
        TaskQueue = setmetatable(TaskQueue, mt)
    end

    do -- Metatype for class instances
        local t  = ffi.typeof('TaskQueue')
        local mt = {
            __index = {
                startWorker      = libphx.TaskQueue_StartWorker,
                stopWorker       = libphx.TaskQueue_StopWorker,
                isWorkerFinished = libphx.TaskQueue_IsWorkerFinished,
                stopAllWorkers   = libphx.TaskQueue_StopAllWorkers,
                tasksInProgress  = libphx.TaskQueue_TasksInProgress,
                sendTask         = libphx.TaskQueue_SendTask,
                nextTaskResult   = function(...)
                    local instance = libphx.TaskQueue_NextTaskResult(...)
                    return Core.ManagedObject(instance, libphx.TaskResult_Free)
                end,
                sendEcho         = libphx.TaskQueue_SendEcho,
                getEcho          = libphx.TaskQueue_GetEcho,
            },
        }

        if onDef_TaskQueue_t then onDef_TaskQueue_t(t, mt) end
        TaskQueue_t = ffi.metatype(t, mt)
    end

    return TaskQueue
end

return Loader
