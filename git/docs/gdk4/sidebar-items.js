initSidebarItems({"constant":[["ACTION_ALL","Defines all possible DND actions."],["BUTTON_MIDDLE","The middle button."],["BUTTON_PRIMARY","The primary button. This is typically the left mouse button, or the right button in a left-handed setup."],["BUTTON_SECONDARY","The secondary button. This is typically the right mouse button, or the left button in a left-handed setup."],["CURRENT_TIME","Represents the current time, and can be used anywhere a time is expected."],["EVENT_PROPAGATE","Use this macro as the return value for continuing the propagation of an event handler."],["EVENT_STOP","Use this macro as the return value for stopping the propagation of an event handler."],["MODIFIER_MASK","A mask covering all entries in [`ModifierType`][crate::ModifierType]."],["NONE_APP_LAUNCH_CONTEXT",""],["NONE_CONTENT_PROVIDER",""],["NONE_DEVICE",""],["NONE_DEVICE_PAD",""],["NONE_DISPLAY",""],["NONE_DRAG",""],["NONE_DRAG_SURFACE",""],["NONE_DRAW_CONTEXT",""],["NONE_EVENT",""],["NONE_GL_CONTEXT",""],["NONE_MONITOR",""],["NONE_PAINTABLE",""],["NONE_POPUP",""],["NONE_SEAT",""],["NONE_SURFACE",""],["NONE_TEXTURE",""],["NONE_TOPLEVEL",""],["PRIORITY_REDRAW","This is the priority that the idle handler processing surface updates is given in the main loop."]],"enum":[["AxisUse","Defines how device axes are interpreted by GTK."],["CrossingMode","Specifies the crossing mode for enter and leave events."],["DevicePadFeature","A pad feature."],["DeviceToolType","Indicates the specific type of tool being used being a tablet. Such as an airbrush, pencil, etc."],["DragCancelReason","Used in [`Drag`][crate::Drag] to the reason of a cancelled DND operation."],["EventType","Specifies the type of the event."],["FullscreenMode","Indicates which monitor a surface should span over when in fullscreen mode."],["GLError","Error enumeration for [`GLContext`][crate::GLContext]."],["Gravity","Defines the reference point of a surface and is used in [`PopupLayout`][crate::PopupLayout]."],["InputSource","An enumeration describing the type of an input device in general terms."],["KeyMatch","Describes how well an event matches a given keyval and modifiers."],["MemoryFormat","[`MemoryFormat`][crate::MemoryFormat] describes a format that bytes can have in memory."],["NotifyType","Specifies the kind of crossing for enter and leave events."],["ScrollDirection","Specifies the direction for scroll events."],["SubpixelLayout","This enumeration describes how the red, green and blue components of physical pixels on an output device are laid out."],["SurfaceEdge","Determines a surface edge or corner."],["TitlebarGesture",""],["TouchpadGesturePhase","Specifies the current state of a touchpad gesture."],["VulkanError","Error enumeration for [`VulkanContext`][crate::VulkanContext]."]],"fn":[["content_deserialize_async","Read content from the given input stream and deserialize it, asynchronously."],["content_deserialize_async_future",""],["content_register_deserializer","Registers a function to deserialize object of a given type."],["content_register_serializer","Registers a function to serialize objects of a given type."],["content_serialize_async","Serialize content and write it to the given output stream, asynchronously."],["content_serialize_async_future",""],["intern_mime_type","Canonicalizes the given mime type and interns the result."],["pango_layout_get_clip_region","Obtains a clip region which contains the areas where the given ranges of text would be drawn."],["pango_layout_line_get_clip_region","Obtains a clip region which contains the areas where the given ranges of text would be drawn."],["pixbuf_get_from_surface","Transfers image data from a `cairo_surface_t` and converts it to a [`gdk_pixbuf::Pixbuf`][crate::gdk_pixbuf::Pixbuf]."],["pixbuf_get_from_texture","Creates a new pixbuf from `texture`."],["set_allowed_backends","Sets a list of backends that GDK should try to use."],["unicode_to_keyval","Convert from a Unicode character to a key symbol."]],"mod":[["functions",""],["keys",""],["prelude","Traits intended for blanket imports."],["subclass",""]],"struct":[["AnchorHints","Positioning hints for aligning a surface relative to a rectangle."],["AppLaunchContext","[`AppLaunchContext`][crate::AppLaunchContext] handles launching an application in a graphical context."],["AxisFlags","Flags describing the current capabilities of a device/tool."],["ButtonEvent","An event related to a button on a pointer device."],["CairoContext","[`CairoContext`][crate::CairoContext] is an object representing the platform-specific draw context."],["Clipboard","The [`Clipboard`][crate::Clipboard] object represents data shared between applications or inside an application."],["ClipboardBuilder","A builder-pattern type to construct [`Clipboard`] objects."],["ContentDeserializer","A [`ContentDeserializer`][crate::ContentDeserializer] is used to deserialize content received via inter-application data transfers."],["ContentFormats","The [`ContentFormats`][crate::ContentFormats] structure is used to advertise and negotiate the format of content."],["ContentFormatsBuilder","A [`ContentFormatsBuilder`][crate::ContentFormatsBuilder] is an auxiliary struct used to create new [`ContentFormats`][crate::ContentFormats], and should not be kept around."],["ContentProvider","A [`ContentProvider`][crate::ContentProvider] is used to provide content for the clipboard or for drag-and-drop operations in a number of formats."],["ContentSerializer","A [`ContentSerializer`][crate::ContentSerializer] is used to serialize content for inter-application data transfers."],["CrossingEvent","An event caused by a pointing device moving between surfaces."],["Cursor","[`Cursor`][crate::Cursor] is used to create and destroy cursors."],["CursorBuilder","A builder-pattern type to construct [`Cursor`] objects."],["DNDEvent","An event related to drag and drop operations."],["DeleteEvent","An event related to closing a top-level surface."],["Device","The [`Device`][crate::Device] object represents an input device, such as a keyboard, a mouse, or a touchpad."],["DevicePad","[`DevicePad`][crate::DevicePad] is an interface implemented by devices of type [`InputSource::TabletPad`][crate::InputSource::TabletPad]"],["DeviceTool","A physical tool associated to a [`Device`][crate::Device]."],["DeviceToolBuilder","A builder-pattern type to construct [`DeviceTool`] objects."],["Display","[`Display`][crate::Display] objects are the GDK representation of a workstation."],["DisplayManager","A singleton object that offers notification when displays appear or disappear."],["Drag","The [`Drag`][crate::Drag] object represents the source of an ongoing DND operation."],["DragAction","Used in [`Drop`][crate::Drop] and [`Drag`][crate::Drag] to indicate the actions that the destination can and should do with the dropped data."],["DragSurface","A [`DragSurface`][crate::DragSurface] is an interface for surfaces used during DND."],["DrawContext","Base class for objects implementing different rendering methods."],["Drop","The [`Drop`][crate::Drop] object represents the target of an ongoing DND operation."],["Event","[`Event`][crate::Event]s are immutable data structures, created by GDK to represent windowing system events."],["EventSequence","[`EventSequence`][crate::EventSequence] is an opaque type representing a sequence of related touch events."],["FocusEvent","An event related to a keyboard focus change."],["FrameClock","A [`FrameClock`][crate::FrameClock] tells the application when to update and repaint a surface."],["FrameClockPhase","Used to represent the different paint clock phases that can be requested."],["FrameTimings","A [`FrameTimings`][crate::FrameTimings] object holds timing information for a single frame of the application’s displays."],["GLContext","[`GLContext`][crate::GLContext] is an object representing a platform-specific OpenGL draw context."],["GLTexture","A GdkTexture representing a GL texture object."],["GRange",""],["GrabBrokenEvent","An event related to a broken windowing system grab."],["KeyEvent","An event related to a key-based device."],["KeymapKey","A [`KeymapKey`][crate::KeymapKey] is a hardware key that can be mapped to a keyval."],["MemoryTexture","A [`Texture`][crate::Texture] representing image data in memory."],["ModifierType","Flags to indicate the state of modifier keys and mouse buttons in events."],["Monitor","[`Monitor`][crate::Monitor] objects represent the individual outputs that are associated with a [`Display`][crate::Display]."],["MotionEvent","An event related to a pointer or touch device motion."],["PadEvent","An event related to a pad-based device."],["Paintable","[`Paintable`][crate::Paintable] is a simple interface used by GTK to represent content that can be painted."],["PaintableFlags","Flags about a paintable object."],["Popup","A [`Popup`][crate::Popup] is a surface that is attached to another surface."],["PopupLayout","The [`PopupLayout`][crate::PopupLayout] struct contains information that is necessary position a [`Popup`][crate::Popup] relative to its parent."],["ProximityEvent","An event related to the proximity of a tool to a device."],["RGBA","A [`RGBA`][crate::RGBA] is used to represent a color, in a way that is compatible with cairo’s notion of color."],["RGBABuilder",""],["Rectangle","A [`Rectangle`][crate::Rectangle] data type for representing rectangles."],["ScrollEvent","An event related to a scrolling motion."],["Seat","The [`Seat`][crate::Seat] object represents a collection of input devices that belong to a user."],["SeatCapabilities","Flags describing the seat capabilities."],["Snapshot","Base type for snapshot operations."],["Surface","A [`Surface`][crate::Surface] is a rectangular region on the screen."],["Texture","[`Texture`][crate::Texture] is the basic element used to refer to pixel data."],["TimeCoord","A [`TimeCoord`][crate::TimeCoord] stores a single event in a motion history."],["Toplevel","A [`Toplevel`][crate::Toplevel] is a freestanding toplevel surface."],["ToplevelLayout","The [`ToplevelLayout`][crate::ToplevelLayout] struct contains information that is necessary to present a sovereign window on screen."],["ToplevelSize","The [`ToplevelSize`][crate::ToplevelSize] struct contains information that is useful to compute the size of a toplevel."],["ToplevelState","Specifies the state of a toplevel surface."],["TouchEvent","An event related to a touch-based device."],["TouchpadEvent","An event related to a gesture on a touchpad device."],["VulkanContext","[`VulkanContext`][crate::VulkanContext] is an object representing the platform-specific Vulkan draw context."]],"trait":[["EventKind",""]]});