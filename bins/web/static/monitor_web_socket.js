/*eslint-disable block-scoped-var, id-length, no-control-regex, no-magic-numbers, no-prototype-builtins, no-redeclare, no-shadow, no-var, sort-vars*/
(function($protobuf) {
    "use strict";

    // Common aliases
    var $Reader = $protobuf.Reader, $Writer = $protobuf.Writer, $util = $protobuf.util;
    
    // Exported root namespace
    var $root = $protobuf.roots["default"] || ($protobuf.roots["default"] = {});
    
    $root.monitor_web_socket = (function() {
    
        /**
         * Namespace monitor_web_socket.
         * @exports monitor_web_socket
         * @namespace
         */
        var monitor_web_socket = {};
    
        monitor_web_socket.ToServer = (function() {
    
            /**
             * Properties of a ToServer.
             * @memberof monitor_web_socket
             * @interface IToServer
             * @property {monitor_web_socket.ISubscribeToMetrics|null} [subscribeToMetrics] ToServer subscribeToMetrics
             */
    
            /**
             * Constructs a new ToServer.
             * @memberof monitor_web_socket
             * @classdesc Represents a ToServer.
             * @implements IToServer
             * @constructor
             * @param {monitor_web_socket.IToServer=} [properties] Properties to set
             */
            function ToServer(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * ToServer subscribeToMetrics.
             * @member {monitor_web_socket.ISubscribeToMetrics|null|undefined} subscribeToMetrics
             * @memberof monitor_web_socket.ToServer
             * @instance
             */
            ToServer.prototype.subscribeToMetrics = null;
    
            // OneOf field names bound to virtual getters and setters
            var $oneOfFields;
    
            /**
             * ToServer msg.
             * @member {"subscribeToMetrics"|undefined} msg
             * @memberof monitor_web_socket.ToServer
             * @instance
             */
            Object.defineProperty(ToServer.prototype, "msg", {
                get: $util.oneOfGetter($oneOfFields = ["subscribeToMetrics"]),
                set: $util.oneOfSetter($oneOfFields)
            });
    
            /**
             * Creates a new ToServer instance using the specified properties.
             * @function create
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {monitor_web_socket.IToServer=} [properties] Properties to set
             * @returns {monitor_web_socket.ToServer} ToServer instance
             */
            ToServer.create = function create(properties) {
                return new ToServer(properties);
            };
    
            /**
             * Encodes the specified ToServer message. Does not implicitly {@link monitor_web_socket.ToServer.verify|verify} messages.
             * @function encode
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {monitor_web_socket.IToServer} message ToServer message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ToServer.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.subscribeToMetrics != null && Object.hasOwnProperty.call(message, "subscribeToMetrics"))
                    $root.monitor_web_socket.SubscribeToMetrics.encode(message.subscribeToMetrics, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                return writer;
            };
    
            /**
             * Encodes the specified ToServer message, length delimited. Does not implicitly {@link monitor_web_socket.ToServer.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {monitor_web_socket.IToServer} message ToServer message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ToServer.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes a ToServer message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_web_socket.ToServer} ToServer
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ToServer.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_web_socket.ToServer();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 2:
                        message.subscribeToMetrics = $root.monitor_web_socket.SubscribeToMetrics.decode(reader, reader.uint32());
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes a ToServer message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_web_socket.ToServer} ToServer
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ToServer.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies a ToServer message.
             * @function verify
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            ToServer.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                var properties = {};
                if (message.subscribeToMetrics != null && message.hasOwnProperty("subscribeToMetrics")) {
                    properties.msg = 1;
                    {
                        var error = $root.monitor_web_socket.SubscribeToMetrics.verify(message.subscribeToMetrics);
                        if (error)
                            return "subscribeToMetrics." + error;
                    }
                }
                return null;
            };
    
            /**
             * Creates a ToServer message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_web_socket.ToServer} ToServer
             */
            ToServer.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_web_socket.ToServer)
                    return object;
                var message = new $root.monitor_web_socket.ToServer();
                if (object.subscribeToMetrics != null) {
                    if (typeof object.subscribeToMetrics !== "object")
                        throw TypeError(".monitor_web_socket.ToServer.subscribeToMetrics: object expected");
                    message.subscribeToMetrics = $root.monitor_web_socket.SubscribeToMetrics.fromObject(object.subscribeToMetrics);
                }
                return message;
            };
    
            /**
             * Creates a plain object from a ToServer message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {monitor_web_socket.ToServer} message ToServer
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            ToServer.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (message.subscribeToMetrics != null && message.hasOwnProperty("subscribeToMetrics")) {
                    object.subscribeToMetrics = $root.monitor_web_socket.SubscribeToMetrics.toObject(message.subscribeToMetrics, options);
                    if (options.oneofs)
                        object.msg = "subscribeToMetrics";
                }
                return object;
            };
    
            /**
             * Converts this ToServer to JSON.
             * @function toJSON
             * @memberof monitor_web_socket.ToServer
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            ToServer.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return ToServer;
        })();
    
        monitor_web_socket.SubscribeToMetrics = (function() {
    
            /**
             * Properties of a SubscribeToMetrics.
             * @memberof monitor_web_socket
             * @interface ISubscribeToMetrics
             */
    
            /**
             * Constructs a new SubscribeToMetrics.
             * @memberof monitor_web_socket
             * @classdesc Represents a SubscribeToMetrics.
             * @implements ISubscribeToMetrics
             * @constructor
             * @param {monitor_web_socket.ISubscribeToMetrics=} [properties] Properties to set
             */
            function SubscribeToMetrics(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * Creates a new SubscribeToMetrics instance using the specified properties.
             * @function create
             * @memberof monitor_web_socket.SubscribeToMetrics
             * @static
             * @param {monitor_web_socket.ISubscribeToMetrics=} [properties] Properties to set
             * @returns {monitor_web_socket.SubscribeToMetrics} SubscribeToMetrics instance
             */
            SubscribeToMetrics.create = function create(properties) {
                return new SubscribeToMetrics(properties);
            };
    
            /**
             * Encodes the specified SubscribeToMetrics message. Does not implicitly {@link monitor_web_socket.SubscribeToMetrics.verify|verify} messages.
             * @function encode
             * @memberof monitor_web_socket.SubscribeToMetrics
             * @static
             * @param {monitor_web_socket.ISubscribeToMetrics} message SubscribeToMetrics message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            SubscribeToMetrics.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                return writer;
            };
    
            /**
             * Encodes the specified SubscribeToMetrics message, length delimited. Does not implicitly {@link monitor_web_socket.SubscribeToMetrics.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_web_socket.SubscribeToMetrics
             * @static
             * @param {monitor_web_socket.ISubscribeToMetrics} message SubscribeToMetrics message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            SubscribeToMetrics.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes a SubscribeToMetrics message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_web_socket.SubscribeToMetrics
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_web_socket.SubscribeToMetrics} SubscribeToMetrics
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            SubscribeToMetrics.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_web_socket.SubscribeToMetrics();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes a SubscribeToMetrics message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_web_socket.SubscribeToMetrics
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_web_socket.SubscribeToMetrics} SubscribeToMetrics
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            SubscribeToMetrics.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies a SubscribeToMetrics message.
             * @function verify
             * @memberof monitor_web_socket.SubscribeToMetrics
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            SubscribeToMetrics.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                return null;
            };
    
            /**
             * Creates a SubscribeToMetrics message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_web_socket.SubscribeToMetrics
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_web_socket.SubscribeToMetrics} SubscribeToMetrics
             */
            SubscribeToMetrics.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_web_socket.SubscribeToMetrics)
                    return object;
                return new $root.monitor_web_socket.SubscribeToMetrics();
            };
    
            /**
             * Creates a plain object from a SubscribeToMetrics message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_web_socket.SubscribeToMetrics
             * @static
             * @param {monitor_web_socket.SubscribeToMetrics} message SubscribeToMetrics
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            SubscribeToMetrics.toObject = function toObject() {
                return {};
            };
    
            /**
             * Converts this SubscribeToMetrics to JSON.
             * @function toJSON
             * @memberof monitor_web_socket.SubscribeToMetrics
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            SubscribeToMetrics.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return SubscribeToMetrics;
        })();
    
        monitor_web_socket.ToClient = (function() {
    
            /**
             * Properties of a ToClient.
             * @memberof monitor_web_socket
             * @interface IToClient
             * @property {monitor_web_socket.IMetricUpdate|null} [metricUpdate] ToClient metricUpdate
             */
    
            /**
             * Constructs a new ToClient.
             * @memberof monitor_web_socket
             * @classdesc Represents a ToClient.
             * @implements IToClient
             * @constructor
             * @param {monitor_web_socket.IToClient=} [properties] Properties to set
             */
            function ToClient(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * ToClient metricUpdate.
             * @member {monitor_web_socket.IMetricUpdate|null|undefined} metricUpdate
             * @memberof monitor_web_socket.ToClient
             * @instance
             */
            ToClient.prototype.metricUpdate = null;
    
            // OneOf field names bound to virtual getters and setters
            var $oneOfFields;
    
            /**
             * ToClient msg.
             * @member {"metricUpdate"|undefined} msg
             * @memberof monitor_web_socket.ToClient
             * @instance
             */
            Object.defineProperty(ToClient.prototype, "msg", {
                get: $util.oneOfGetter($oneOfFields = ["metricUpdate"]),
                set: $util.oneOfSetter($oneOfFields)
            });
    
            /**
             * Creates a new ToClient instance using the specified properties.
             * @function create
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {monitor_web_socket.IToClient=} [properties] Properties to set
             * @returns {monitor_web_socket.ToClient} ToClient instance
             */
            ToClient.create = function create(properties) {
                return new ToClient(properties);
            };
    
            /**
             * Encodes the specified ToClient message. Does not implicitly {@link monitor_web_socket.ToClient.verify|verify} messages.
             * @function encode
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {monitor_web_socket.IToClient} message ToClient message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ToClient.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.metricUpdate != null && Object.hasOwnProperty.call(message, "metricUpdate"))
                    $root.monitor_web_socket.MetricUpdate.encode(message.metricUpdate, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                return writer;
            };
    
            /**
             * Encodes the specified ToClient message, length delimited. Does not implicitly {@link monitor_web_socket.ToClient.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {monitor_web_socket.IToClient} message ToClient message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ToClient.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes a ToClient message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_web_socket.ToClient} ToClient
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ToClient.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_web_socket.ToClient();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 2:
                        message.metricUpdate = $root.monitor_web_socket.MetricUpdate.decode(reader, reader.uint32());
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes a ToClient message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_web_socket.ToClient} ToClient
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ToClient.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies a ToClient message.
             * @function verify
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            ToClient.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                var properties = {};
                if (message.metricUpdate != null && message.hasOwnProperty("metricUpdate")) {
                    properties.msg = 1;
                    {
                        var error = $root.monitor_web_socket.MetricUpdate.verify(message.metricUpdate);
                        if (error)
                            return "metricUpdate." + error;
                    }
                }
                return null;
            };
    
            /**
             * Creates a ToClient message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_web_socket.ToClient} ToClient
             */
            ToClient.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_web_socket.ToClient)
                    return object;
                var message = new $root.monitor_web_socket.ToClient();
                if (object.metricUpdate != null) {
                    if (typeof object.metricUpdate !== "object")
                        throw TypeError(".monitor_web_socket.ToClient.metricUpdate: object expected");
                    message.metricUpdate = $root.monitor_web_socket.MetricUpdate.fromObject(object.metricUpdate);
                }
                return message;
            };
    
            /**
             * Creates a plain object from a ToClient message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {monitor_web_socket.ToClient} message ToClient
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            ToClient.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (message.metricUpdate != null && message.hasOwnProperty("metricUpdate")) {
                    object.metricUpdate = $root.monitor_web_socket.MetricUpdate.toObject(message.metricUpdate, options);
                    if (options.oneofs)
                        object.msg = "metricUpdate";
                }
                return object;
            };
    
            /**
             * Converts this ToClient to JSON.
             * @function toJSON
             * @memberof monitor_web_socket.ToClient
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            ToClient.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return ToClient;
        })();
    
        monitor_web_socket.MetricUpdate = (function() {
    
            /**
             * Properties of a MetricUpdate.
             * @memberof monitor_web_socket
             * @interface IMetricUpdate
             * @property {monitor_core_types.IMetric|null} [metric] MetricUpdate metric
             */
    
            /**
             * Constructs a new MetricUpdate.
             * @memberof monitor_web_socket
             * @classdesc Represents a MetricUpdate.
             * @implements IMetricUpdate
             * @constructor
             * @param {monitor_web_socket.IMetricUpdate=} [properties] Properties to set
             */
            function MetricUpdate(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * MetricUpdate metric.
             * @member {monitor_core_types.IMetric|null|undefined} metric
             * @memberof monitor_web_socket.MetricUpdate
             * @instance
             */
            MetricUpdate.prototype.metric = null;
    
            /**
             * Creates a new MetricUpdate instance using the specified properties.
             * @function create
             * @memberof monitor_web_socket.MetricUpdate
             * @static
             * @param {monitor_web_socket.IMetricUpdate=} [properties] Properties to set
             * @returns {monitor_web_socket.MetricUpdate} MetricUpdate instance
             */
            MetricUpdate.create = function create(properties) {
                return new MetricUpdate(properties);
            };
    
            /**
             * Encodes the specified MetricUpdate message. Does not implicitly {@link monitor_web_socket.MetricUpdate.verify|verify} messages.
             * @function encode
             * @memberof monitor_web_socket.MetricUpdate
             * @static
             * @param {monitor_web_socket.IMetricUpdate} message MetricUpdate message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            MetricUpdate.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.metric != null && Object.hasOwnProperty.call(message, "metric"))
                    $root.monitor_core_types.Metric.encode(message.metric, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                return writer;
            };
    
            /**
             * Encodes the specified MetricUpdate message, length delimited. Does not implicitly {@link monitor_web_socket.MetricUpdate.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_web_socket.MetricUpdate
             * @static
             * @param {monitor_web_socket.IMetricUpdate} message MetricUpdate message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            MetricUpdate.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes a MetricUpdate message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_web_socket.MetricUpdate
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_web_socket.MetricUpdate} MetricUpdate
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            MetricUpdate.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_web_socket.MetricUpdate();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 1:
                        message.metric = $root.monitor_core_types.Metric.decode(reader, reader.uint32());
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes a MetricUpdate message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_web_socket.MetricUpdate
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_web_socket.MetricUpdate} MetricUpdate
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            MetricUpdate.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies a MetricUpdate message.
             * @function verify
             * @memberof monitor_web_socket.MetricUpdate
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            MetricUpdate.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.metric != null && message.hasOwnProperty("metric")) {
                    var error = $root.monitor_core_types.Metric.verify(message.metric);
                    if (error)
                        return "metric." + error;
                }
                return null;
            };
    
            /**
             * Creates a MetricUpdate message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_web_socket.MetricUpdate
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_web_socket.MetricUpdate} MetricUpdate
             */
            MetricUpdate.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_web_socket.MetricUpdate)
                    return object;
                var message = new $root.monitor_web_socket.MetricUpdate();
                if (object.metric != null) {
                    if (typeof object.metric !== "object")
                        throw TypeError(".monitor_web_socket.MetricUpdate.metric: object expected");
                    message.metric = $root.monitor_core_types.Metric.fromObject(object.metric);
                }
                return message;
            };
    
            /**
             * Creates a plain object from a MetricUpdate message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_web_socket.MetricUpdate
             * @static
             * @param {monitor_web_socket.MetricUpdate} message MetricUpdate
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            MetricUpdate.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (options.defaults)
                    object.metric = null;
                if (message.metric != null && message.hasOwnProperty("metric"))
                    object.metric = $root.monitor_core_types.Metric.toObject(message.metric, options);
                return object;
            };
    
            /**
             * Converts this MetricUpdate to JSON.
             * @function toJSON
             * @memberof monitor_web_socket.MetricUpdate
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            MetricUpdate.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return MetricUpdate;
        })();
    
        return monitor_web_socket;
    })();
    
    $root.monitor_core_types = (function() {
    
        /**
         * Namespace monitor_core_types.
         * @exports monitor_core_types
         * @namespace
         */
        var monitor_core_types = {};
    
        monitor_core_types.Metric = (function() {
    
            /**
             * Properties of a Metric.
             * @memberof monitor_core_types
             * @interface IMetric
             * @property {monitor_core_types.IMetricKey|null} [key] Metric key
             * @property {monitor_core_types.IDataPoint|null} [latest] Metric latest
             */
    
            /**
             * Constructs a new Metric.
             * @memberof monitor_core_types
             * @classdesc Represents a Metric.
             * @implements IMetric
             * @constructor
             * @param {monitor_core_types.IMetric=} [properties] Properties to set
             */
            function Metric(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * Metric key.
             * @member {monitor_core_types.IMetricKey|null|undefined} key
             * @memberof monitor_core_types.Metric
             * @instance
             */
            Metric.prototype.key = null;
    
            /**
             * Metric latest.
             * @member {monitor_core_types.IDataPoint|null|undefined} latest
             * @memberof monitor_core_types.Metric
             * @instance
             */
            Metric.prototype.latest = null;
    
            /**
             * Creates a new Metric instance using the specified properties.
             * @function create
             * @memberof monitor_core_types.Metric
             * @static
             * @param {monitor_core_types.IMetric=} [properties] Properties to set
             * @returns {monitor_core_types.Metric} Metric instance
             */
            Metric.create = function create(properties) {
                return new Metric(properties);
            };
    
            /**
             * Encodes the specified Metric message. Does not implicitly {@link monitor_core_types.Metric.verify|verify} messages.
             * @function encode
             * @memberof monitor_core_types.Metric
             * @static
             * @param {monitor_core_types.IMetric} message Metric message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Metric.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.latest != null && Object.hasOwnProperty.call(message, "latest"))
                    $root.monitor_core_types.DataPoint.encode(message.latest, writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
                if (message.key != null && Object.hasOwnProperty.call(message, "key"))
                    $root.monitor_core_types.MetricKey.encode(message.key, writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
                return writer;
            };
    
            /**
             * Encodes the specified Metric message, length delimited. Does not implicitly {@link monitor_core_types.Metric.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_core_types.Metric
             * @static
             * @param {monitor_core_types.IMetric} message Metric message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Metric.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes a Metric message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_core_types.Metric
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_core_types.Metric} Metric
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Metric.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_core_types.Metric();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 4:
                        message.key = $root.monitor_core_types.MetricKey.decode(reader, reader.uint32());
                        break;
                    case 3:
                        message.latest = $root.monitor_core_types.DataPoint.decode(reader, reader.uint32());
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes a Metric message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_core_types.Metric
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_core_types.Metric} Metric
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Metric.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies a Metric message.
             * @function verify
             * @memberof monitor_core_types.Metric
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Metric.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.key != null && message.hasOwnProperty("key")) {
                    var error = $root.monitor_core_types.MetricKey.verify(message.key);
                    if (error)
                        return "key." + error;
                }
                if (message.latest != null && message.hasOwnProperty("latest")) {
                    var error = $root.monitor_core_types.DataPoint.verify(message.latest);
                    if (error)
                        return "latest." + error;
                }
                return null;
            };
    
            /**
             * Creates a Metric message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_core_types.Metric
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_core_types.Metric} Metric
             */
            Metric.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_core_types.Metric)
                    return object;
                var message = new $root.monitor_core_types.Metric();
                if (object.key != null) {
                    if (typeof object.key !== "object")
                        throw TypeError(".monitor_core_types.Metric.key: object expected");
                    message.key = $root.monitor_core_types.MetricKey.fromObject(object.key);
                }
                if (object.latest != null) {
                    if (typeof object.latest !== "object")
                        throw TypeError(".monitor_core_types.Metric.latest: object expected");
                    message.latest = $root.monitor_core_types.DataPoint.fromObject(object.latest);
                }
                return message;
            };
    
            /**
             * Creates a plain object from a Metric message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_core_types.Metric
             * @static
             * @param {monitor_core_types.Metric} message Metric
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Metric.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (options.defaults) {
                    object.latest = null;
                    object.key = null;
                }
                if (message.latest != null && message.hasOwnProperty("latest"))
                    object.latest = $root.monitor_core_types.DataPoint.toObject(message.latest, options);
                if (message.key != null && message.hasOwnProperty("key"))
                    object.key = $root.monitor_core_types.MetricKey.toObject(message.key, options);
                return object;
            };
    
            /**
             * Converts this Metric to JSON.
             * @function toJSON
             * @memberof monitor_core_types.Metric
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Metric.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return Metric;
        })();
    
        monitor_core_types.DataPoint = (function() {
    
            /**
             * Properties of a DataPoint.
             * @memberof monitor_core_types
             * @interface IDataPoint
             * @property {monitor_core_types.ITime|null} [time] DataPoint time
             * @property {number|Long|null} [i64] DataPoint i64
             * @property {number|null} [f64] DataPoint f64
             * @property {monitor_core_types.INone|null} [none] DataPoint none
             * @property {boolean|null} [ok] DataPoint ok
             */
    
            /**
             * Constructs a new DataPoint.
             * @memberof monitor_core_types
             * @classdesc Represents a DataPoint.
             * @implements IDataPoint
             * @constructor
             * @param {monitor_core_types.IDataPoint=} [properties] Properties to set
             */
            function DataPoint(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * DataPoint time.
             * @member {monitor_core_types.ITime|null|undefined} time
             * @memberof monitor_core_types.DataPoint
             * @instance
             */
            DataPoint.prototype.time = null;
    
            /**
             * DataPoint i64.
             * @member {number|Long} i64
             * @memberof monitor_core_types.DataPoint
             * @instance
             */
            DataPoint.prototype.i64 = $util.Long ? $util.Long.fromBits(0,0,false) : 0;
    
            /**
             * DataPoint f64.
             * @member {number} f64
             * @memberof monitor_core_types.DataPoint
             * @instance
             */
            DataPoint.prototype.f64 = 0;
    
            /**
             * DataPoint none.
             * @member {monitor_core_types.INone|null|undefined} none
             * @memberof monitor_core_types.DataPoint
             * @instance
             */
            DataPoint.prototype.none = null;
    
            /**
             * DataPoint ok.
             * @member {boolean} ok
             * @memberof monitor_core_types.DataPoint
             * @instance
             */
            DataPoint.prototype.ok = false;
    
            // OneOf field names bound to virtual getters and setters
            var $oneOfFields;
    
            /**
             * DataPoint value.
             * @member {"i64"|"f64"|"none"|undefined} value
             * @memberof monitor_core_types.DataPoint
             * @instance
             */
            Object.defineProperty(DataPoint.prototype, "value", {
                get: $util.oneOfGetter($oneOfFields = ["i64", "f64", "none"]),
                set: $util.oneOfSetter($oneOfFields)
            });
    
            /**
             * Creates a new DataPoint instance using the specified properties.
             * @function create
             * @memberof monitor_core_types.DataPoint
             * @static
             * @param {monitor_core_types.IDataPoint=} [properties] Properties to set
             * @returns {monitor_core_types.DataPoint} DataPoint instance
             */
            DataPoint.create = function create(properties) {
                return new DataPoint(properties);
            };
    
            /**
             * Encodes the specified DataPoint message. Does not implicitly {@link monitor_core_types.DataPoint.verify|verify} messages.
             * @function encode
             * @memberof monitor_core_types.DataPoint
             * @static
             * @param {monitor_core_types.IDataPoint} message DataPoint message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            DataPoint.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.time != null && Object.hasOwnProperty.call(message, "time"))
                    $root.monitor_core_types.Time.encode(message.time, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                if (message.i64 != null && Object.hasOwnProperty.call(message, "i64"))
                    writer.uint32(/* id 3, wireType 0 =*/24).int64(message.i64);
                if (message.f64 != null && Object.hasOwnProperty.call(message, "f64"))
                    writer.uint32(/* id 4, wireType 1 =*/33).double(message.f64);
                if (message.ok != null && Object.hasOwnProperty.call(message, "ok"))
                    writer.uint32(/* id 5, wireType 0 =*/40).bool(message.ok);
                if (message.none != null && Object.hasOwnProperty.call(message, "none"))
                    $root.monitor_core_types.None.encode(message.none, writer.uint32(/* id 6, wireType 2 =*/50).fork()).ldelim();
                return writer;
            };
    
            /**
             * Encodes the specified DataPoint message, length delimited. Does not implicitly {@link monitor_core_types.DataPoint.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_core_types.DataPoint
             * @static
             * @param {monitor_core_types.IDataPoint} message DataPoint message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            DataPoint.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes a DataPoint message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_core_types.DataPoint
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_core_types.DataPoint} DataPoint
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            DataPoint.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_core_types.DataPoint();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 1:
                        message.time = $root.monitor_core_types.Time.decode(reader, reader.uint32());
                        break;
                    case 3:
                        message.i64 = reader.int64();
                        break;
                    case 4:
                        message.f64 = reader.double();
                        break;
                    case 6:
                        message.none = $root.monitor_core_types.None.decode(reader, reader.uint32());
                        break;
                    case 5:
                        message.ok = reader.bool();
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes a DataPoint message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_core_types.DataPoint
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_core_types.DataPoint} DataPoint
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            DataPoint.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies a DataPoint message.
             * @function verify
             * @memberof monitor_core_types.DataPoint
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            DataPoint.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                var properties = {};
                if (message.time != null && message.hasOwnProperty("time")) {
                    var error = $root.monitor_core_types.Time.verify(message.time);
                    if (error)
                        return "time." + error;
                }
                if (message.i64 != null && message.hasOwnProperty("i64")) {
                    properties.value = 1;
                    if (!$util.isInteger(message.i64) && !(message.i64 && $util.isInteger(message.i64.low) && $util.isInteger(message.i64.high)))
                        return "i64: integer|Long expected";
                }
                if (message.f64 != null && message.hasOwnProperty("f64")) {
                    if (properties.value === 1)
                        return "value: multiple values";
                    properties.value = 1;
                    if (typeof message.f64 !== "number")
                        return "f64: number expected";
                }
                if (message.none != null && message.hasOwnProperty("none")) {
                    if (properties.value === 1)
                        return "value: multiple values";
                    properties.value = 1;
                    {
                        var error = $root.monitor_core_types.None.verify(message.none);
                        if (error)
                            return "none." + error;
                    }
                }
                if (message.ok != null && message.hasOwnProperty("ok"))
                    if (typeof message.ok !== "boolean")
                        return "ok: boolean expected";
                return null;
            };
    
            /**
             * Creates a DataPoint message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_core_types.DataPoint
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_core_types.DataPoint} DataPoint
             */
            DataPoint.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_core_types.DataPoint)
                    return object;
                var message = new $root.monitor_core_types.DataPoint();
                if (object.time != null) {
                    if (typeof object.time !== "object")
                        throw TypeError(".monitor_core_types.DataPoint.time: object expected");
                    message.time = $root.monitor_core_types.Time.fromObject(object.time);
                }
                if (object.i64 != null)
                    if ($util.Long)
                        (message.i64 = $util.Long.fromValue(object.i64)).unsigned = false;
                    else if (typeof object.i64 === "string")
                        message.i64 = parseInt(object.i64, 10);
                    else if (typeof object.i64 === "number")
                        message.i64 = object.i64;
                    else if (typeof object.i64 === "object")
                        message.i64 = new $util.LongBits(object.i64.low >>> 0, object.i64.high >>> 0).toNumber();
                if (object.f64 != null)
                    message.f64 = Number(object.f64);
                if (object.none != null) {
                    if (typeof object.none !== "object")
                        throw TypeError(".monitor_core_types.DataPoint.none: object expected");
                    message.none = $root.monitor_core_types.None.fromObject(object.none);
                }
                if (object.ok != null)
                    message.ok = Boolean(object.ok);
                return message;
            };
    
            /**
             * Creates a plain object from a DataPoint message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_core_types.DataPoint
             * @static
             * @param {monitor_core_types.DataPoint} message DataPoint
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            DataPoint.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (options.defaults) {
                    object.time = null;
                    object.ok = false;
                }
                if (message.time != null && message.hasOwnProperty("time"))
                    object.time = $root.monitor_core_types.Time.toObject(message.time, options);
                if (message.i64 != null && message.hasOwnProperty("i64")) {
                    if (typeof message.i64 === "number")
                        object.i64 = options.longs === String ? String(message.i64) : message.i64;
                    else
                        object.i64 = options.longs === String ? $util.Long.prototype.toString.call(message.i64) : options.longs === Number ? new $util.LongBits(message.i64.low >>> 0, message.i64.high >>> 0).toNumber() : message.i64;
                    if (options.oneofs)
                        object.value = "i64";
                }
                if (message.f64 != null && message.hasOwnProperty("f64")) {
                    object.f64 = options.json && !isFinite(message.f64) ? String(message.f64) : message.f64;
                    if (options.oneofs)
                        object.value = "f64";
                }
                if (message.ok != null && message.hasOwnProperty("ok"))
                    object.ok = message.ok;
                if (message.none != null && message.hasOwnProperty("none")) {
                    object.none = $root.monitor_core_types.None.toObject(message.none, options);
                    if (options.oneofs)
                        object.value = "none";
                }
                return object;
            };
    
            /**
             * Converts this DataPoint to JSON.
             * @function toJSON
             * @memberof monitor_core_types.DataPoint
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            DataPoint.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return DataPoint;
        })();
    
        monitor_core_types.Log = (function() {
    
            /**
             * Properties of a Log.
             * @memberof monitor_core_types
             * @interface ILog
             * @property {monitor_core_types.ITime|null} [start] Log start
             * @property {monitor_core_types.ITime|null} [finish] Log finish
             * @property {monitor_core_types.IDuration|null} [duration] Log duration
             * @property {string|null} [log] Log log
             * @property {monitor_core_types.IMetricKey|null} [key] Log key
             */
    
            /**
             * Constructs a new Log.
             * @memberof monitor_core_types
             * @classdesc Represents a Log.
             * @implements ILog
             * @constructor
             * @param {monitor_core_types.ILog=} [properties] Properties to set
             */
            function Log(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * Log start.
             * @member {monitor_core_types.ITime|null|undefined} start
             * @memberof monitor_core_types.Log
             * @instance
             */
            Log.prototype.start = null;
    
            /**
             * Log finish.
             * @member {monitor_core_types.ITime|null|undefined} finish
             * @memberof monitor_core_types.Log
             * @instance
             */
            Log.prototype.finish = null;
    
            /**
             * Log duration.
             * @member {monitor_core_types.IDuration|null|undefined} duration
             * @memberof monitor_core_types.Log
             * @instance
             */
            Log.prototype.duration = null;
    
            /**
             * Log log.
             * @member {string} log
             * @memberof monitor_core_types.Log
             * @instance
             */
            Log.prototype.log = "";
    
            /**
             * Log key.
             * @member {monitor_core_types.IMetricKey|null|undefined} key
             * @memberof monitor_core_types.Log
             * @instance
             */
            Log.prototype.key = null;
    
            /**
             * Creates a new Log instance using the specified properties.
             * @function create
             * @memberof monitor_core_types.Log
             * @static
             * @param {monitor_core_types.ILog=} [properties] Properties to set
             * @returns {monitor_core_types.Log} Log instance
             */
            Log.create = function create(properties) {
                return new Log(properties);
            };
    
            /**
             * Encodes the specified Log message. Does not implicitly {@link monitor_core_types.Log.verify|verify} messages.
             * @function encode
             * @memberof monitor_core_types.Log
             * @static
             * @param {monitor_core_types.ILog} message Log message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Log.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.start != null && Object.hasOwnProperty.call(message, "start"))
                    $root.monitor_core_types.Time.encode(message.start, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                if (message.finish != null && Object.hasOwnProperty.call(message, "finish"))
                    $root.monitor_core_types.Time.encode(message.finish, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                if (message.duration != null && Object.hasOwnProperty.call(message, "duration"))
                    $root.monitor_core_types.Duration.encode(message.duration, writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
                if (message.log != null && Object.hasOwnProperty.call(message, "log"))
                    writer.uint32(/* id 4, wireType 2 =*/34).string(message.log);
                if (message.key != null && Object.hasOwnProperty.call(message, "key"))
                    $root.monitor_core_types.MetricKey.encode(message.key, writer.uint32(/* id 5, wireType 2 =*/42).fork()).ldelim();
                return writer;
            };
    
            /**
             * Encodes the specified Log message, length delimited. Does not implicitly {@link monitor_core_types.Log.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_core_types.Log
             * @static
             * @param {monitor_core_types.ILog} message Log message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Log.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes a Log message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_core_types.Log
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_core_types.Log} Log
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Log.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_core_types.Log();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 1:
                        message.start = $root.monitor_core_types.Time.decode(reader, reader.uint32());
                        break;
                    case 2:
                        message.finish = $root.monitor_core_types.Time.decode(reader, reader.uint32());
                        break;
                    case 3:
                        message.duration = $root.monitor_core_types.Duration.decode(reader, reader.uint32());
                        break;
                    case 4:
                        message.log = reader.string();
                        break;
                    case 5:
                        message.key = $root.monitor_core_types.MetricKey.decode(reader, reader.uint32());
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes a Log message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_core_types.Log
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_core_types.Log} Log
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Log.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies a Log message.
             * @function verify
             * @memberof monitor_core_types.Log
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Log.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.start != null && message.hasOwnProperty("start")) {
                    var error = $root.monitor_core_types.Time.verify(message.start);
                    if (error)
                        return "start." + error;
                }
                if (message.finish != null && message.hasOwnProperty("finish")) {
                    var error = $root.monitor_core_types.Time.verify(message.finish);
                    if (error)
                        return "finish." + error;
                }
                if (message.duration != null && message.hasOwnProperty("duration")) {
                    var error = $root.monitor_core_types.Duration.verify(message.duration);
                    if (error)
                        return "duration." + error;
                }
                if (message.log != null && message.hasOwnProperty("log"))
                    if (!$util.isString(message.log))
                        return "log: string expected";
                if (message.key != null && message.hasOwnProperty("key")) {
                    var error = $root.monitor_core_types.MetricKey.verify(message.key);
                    if (error)
                        return "key." + error;
                }
                return null;
            };
    
            /**
             * Creates a Log message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_core_types.Log
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_core_types.Log} Log
             */
            Log.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_core_types.Log)
                    return object;
                var message = new $root.monitor_core_types.Log();
                if (object.start != null) {
                    if (typeof object.start !== "object")
                        throw TypeError(".monitor_core_types.Log.start: object expected");
                    message.start = $root.monitor_core_types.Time.fromObject(object.start);
                }
                if (object.finish != null) {
                    if (typeof object.finish !== "object")
                        throw TypeError(".monitor_core_types.Log.finish: object expected");
                    message.finish = $root.monitor_core_types.Time.fromObject(object.finish);
                }
                if (object.duration != null) {
                    if (typeof object.duration !== "object")
                        throw TypeError(".monitor_core_types.Log.duration: object expected");
                    message.duration = $root.monitor_core_types.Duration.fromObject(object.duration);
                }
                if (object.log != null)
                    message.log = String(object.log);
                if (object.key != null) {
                    if (typeof object.key !== "object")
                        throw TypeError(".monitor_core_types.Log.key: object expected");
                    message.key = $root.monitor_core_types.MetricKey.fromObject(object.key);
                }
                return message;
            };
    
            /**
             * Creates a plain object from a Log message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_core_types.Log
             * @static
             * @param {monitor_core_types.Log} message Log
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Log.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (options.defaults) {
                    object.start = null;
                    object.finish = null;
                    object.duration = null;
                    object.log = "";
                    object.key = null;
                }
                if (message.start != null && message.hasOwnProperty("start"))
                    object.start = $root.monitor_core_types.Time.toObject(message.start, options);
                if (message.finish != null && message.hasOwnProperty("finish"))
                    object.finish = $root.monitor_core_types.Time.toObject(message.finish, options);
                if (message.duration != null && message.hasOwnProperty("duration"))
                    object.duration = $root.monitor_core_types.Duration.toObject(message.duration, options);
                if (message.log != null && message.hasOwnProperty("log"))
                    object.log = message.log;
                if (message.key != null && message.hasOwnProperty("key"))
                    object.key = $root.monitor_core_types.MetricKey.toObject(message.key, options);
                return object;
            };
    
            /**
             * Converts this Log to JSON.
             * @function toJSON
             * @memberof monitor_core_types.Log
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Log.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return Log;
        })();
    
        monitor_core_types.MetricKey = (function() {
    
            /**
             * Properties of a MetricKey.
             * @memberof monitor_core_types
             * @interface IMetricKey
             * @property {string|null} [name] MetricKey name
             * @property {monitor_core_types.IHost|null} [fromHost] MetricKey fromHost
             */
    
            /**
             * Constructs a new MetricKey.
             * @memberof monitor_core_types
             * @classdesc Represents a MetricKey.
             * @implements IMetricKey
             * @constructor
             * @param {monitor_core_types.IMetricKey=} [properties] Properties to set
             */
            function MetricKey(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * MetricKey name.
             * @member {string} name
             * @memberof monitor_core_types.MetricKey
             * @instance
             */
            MetricKey.prototype.name = "";
    
            /**
             * MetricKey fromHost.
             * @member {monitor_core_types.IHost|null|undefined} fromHost
             * @memberof monitor_core_types.MetricKey
             * @instance
             */
            MetricKey.prototype.fromHost = null;
    
            /**
             * Creates a new MetricKey instance using the specified properties.
             * @function create
             * @memberof monitor_core_types.MetricKey
             * @static
             * @param {monitor_core_types.IMetricKey=} [properties] Properties to set
             * @returns {monitor_core_types.MetricKey} MetricKey instance
             */
            MetricKey.create = function create(properties) {
                return new MetricKey(properties);
            };
    
            /**
             * Encodes the specified MetricKey message. Does not implicitly {@link monitor_core_types.MetricKey.verify|verify} messages.
             * @function encode
             * @memberof monitor_core_types.MetricKey
             * @static
             * @param {monitor_core_types.IMetricKey} message MetricKey message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            MetricKey.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.name != null && Object.hasOwnProperty.call(message, "name"))
                    writer.uint32(/* id 1, wireType 2 =*/10).string(message.name);
                if (message.fromHost != null && Object.hasOwnProperty.call(message, "fromHost"))
                    $root.monitor_core_types.Host.encode(message.fromHost, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
                return writer;
            };
    
            /**
             * Encodes the specified MetricKey message, length delimited. Does not implicitly {@link monitor_core_types.MetricKey.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_core_types.MetricKey
             * @static
             * @param {monitor_core_types.IMetricKey} message MetricKey message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            MetricKey.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes a MetricKey message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_core_types.MetricKey
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_core_types.MetricKey} MetricKey
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            MetricKey.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_core_types.MetricKey();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 1:
                        message.name = reader.string();
                        break;
                    case 2:
                        message.fromHost = $root.monitor_core_types.Host.decode(reader, reader.uint32());
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes a MetricKey message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_core_types.MetricKey
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_core_types.MetricKey} MetricKey
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            MetricKey.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies a MetricKey message.
             * @function verify
             * @memberof monitor_core_types.MetricKey
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            MetricKey.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.name != null && message.hasOwnProperty("name"))
                    if (!$util.isString(message.name))
                        return "name: string expected";
                if (message.fromHost != null && message.hasOwnProperty("fromHost")) {
                    var error = $root.monitor_core_types.Host.verify(message.fromHost);
                    if (error)
                        return "fromHost." + error;
                }
                return null;
            };
    
            /**
             * Creates a MetricKey message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_core_types.MetricKey
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_core_types.MetricKey} MetricKey
             */
            MetricKey.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_core_types.MetricKey)
                    return object;
                var message = new $root.monitor_core_types.MetricKey();
                if (object.name != null)
                    message.name = String(object.name);
                if (object.fromHost != null) {
                    if (typeof object.fromHost !== "object")
                        throw TypeError(".monitor_core_types.MetricKey.fromHost: object expected");
                    message.fromHost = $root.monitor_core_types.Host.fromObject(object.fromHost);
                }
                return message;
            };
    
            /**
             * Creates a plain object from a MetricKey message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_core_types.MetricKey
             * @static
             * @param {monitor_core_types.MetricKey} message MetricKey
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            MetricKey.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (options.defaults) {
                    object.name = "";
                    object.fromHost = null;
                }
                if (message.name != null && message.hasOwnProperty("name"))
                    object.name = message.name;
                if (message.fromHost != null && message.hasOwnProperty("fromHost"))
                    object.fromHost = $root.monitor_core_types.Host.toObject(message.fromHost, options);
                return object;
            };
    
            /**
             * Converts this MetricKey to JSON.
             * @function toJSON
             * @memberof monitor_core_types.MetricKey
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            MetricKey.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return MetricKey;
        })();
    
        monitor_core_types.Host = (function() {
    
            /**
             * Properties of a Host.
             * @memberof monitor_core_types
             * @interface IHost
             * @property {string|null} [name] Host name
             */
    
            /**
             * Constructs a new Host.
             * @memberof monitor_core_types
             * @classdesc Represents a Host.
             * @implements IHost
             * @constructor
             * @param {monitor_core_types.IHost=} [properties] Properties to set
             */
            function Host(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * Host name.
             * @member {string} name
             * @memberof monitor_core_types.Host
             * @instance
             */
            Host.prototype.name = "";
    
            /**
             * Creates a new Host instance using the specified properties.
             * @function create
             * @memberof monitor_core_types.Host
             * @static
             * @param {monitor_core_types.IHost=} [properties] Properties to set
             * @returns {monitor_core_types.Host} Host instance
             */
            Host.create = function create(properties) {
                return new Host(properties);
            };
    
            /**
             * Encodes the specified Host message. Does not implicitly {@link monitor_core_types.Host.verify|verify} messages.
             * @function encode
             * @memberof monitor_core_types.Host
             * @static
             * @param {monitor_core_types.IHost} message Host message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Host.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.name != null && Object.hasOwnProperty.call(message, "name"))
                    writer.uint32(/* id 1, wireType 2 =*/10).string(message.name);
                return writer;
            };
    
            /**
             * Encodes the specified Host message, length delimited. Does not implicitly {@link monitor_core_types.Host.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_core_types.Host
             * @static
             * @param {monitor_core_types.IHost} message Host message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Host.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes a Host message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_core_types.Host
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_core_types.Host} Host
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Host.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_core_types.Host();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 1:
                        message.name = reader.string();
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes a Host message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_core_types.Host
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_core_types.Host} Host
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Host.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies a Host message.
             * @function verify
             * @memberof monitor_core_types.Host
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Host.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.name != null && message.hasOwnProperty("name"))
                    if (!$util.isString(message.name))
                        return "name: string expected";
                return null;
            };
    
            /**
             * Creates a Host message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_core_types.Host
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_core_types.Host} Host
             */
            Host.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_core_types.Host)
                    return object;
                var message = new $root.monitor_core_types.Host();
                if (object.name != null)
                    message.name = String(object.name);
                return message;
            };
    
            /**
             * Creates a plain object from a Host message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_core_types.Host
             * @static
             * @param {monitor_core_types.Host} message Host
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Host.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (options.defaults)
                    object.name = "";
                if (message.name != null && message.hasOwnProperty("name"))
                    object.name = message.name;
                return object;
            };
    
            /**
             * Converts this Host to JSON.
             * @function toJSON
             * @memberof monitor_core_types.Host
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Host.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return Host;
        })();
    
        monitor_core_types.Time = (function() {
    
            /**
             * Properties of a Time.
             * @memberof monitor_core_types
             * @interface ITime
             * @property {number|Long|null} [epochMillis] Time epochMillis
             * @property {number|null} [nanos] Time nanos
             */
    
            /**
             * Constructs a new Time.
             * @memberof monitor_core_types
             * @classdesc Represents a Time.
             * @implements ITime
             * @constructor
             * @param {monitor_core_types.ITime=} [properties] Properties to set
             */
            function Time(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * Time epochMillis.
             * @member {number|Long} epochMillis
             * @memberof monitor_core_types.Time
             * @instance
             */
            Time.prototype.epochMillis = $util.Long ? $util.Long.fromBits(0,0,false) : 0;
    
            /**
             * Time nanos.
             * @member {number} nanos
             * @memberof monitor_core_types.Time
             * @instance
             */
            Time.prototype.nanos = 0;
    
            /**
             * Creates a new Time instance using the specified properties.
             * @function create
             * @memberof monitor_core_types.Time
             * @static
             * @param {monitor_core_types.ITime=} [properties] Properties to set
             * @returns {monitor_core_types.Time} Time instance
             */
            Time.create = function create(properties) {
                return new Time(properties);
            };
    
            /**
             * Encodes the specified Time message. Does not implicitly {@link monitor_core_types.Time.verify|verify} messages.
             * @function encode
             * @memberof monitor_core_types.Time
             * @static
             * @param {monitor_core_types.ITime} message Time message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Time.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.epochMillis != null && Object.hasOwnProperty.call(message, "epochMillis"))
                    writer.uint32(/* id 1, wireType 1 =*/9).sfixed64(message.epochMillis);
                if (message.nanos != null && Object.hasOwnProperty.call(message, "nanos"))
                    writer.uint32(/* id 2, wireType 5 =*/21).fixed32(message.nanos);
                return writer;
            };
    
            /**
             * Encodes the specified Time message, length delimited. Does not implicitly {@link monitor_core_types.Time.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_core_types.Time
             * @static
             * @param {monitor_core_types.ITime} message Time message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Time.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes a Time message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_core_types.Time
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_core_types.Time} Time
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Time.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_core_types.Time();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 1:
                        message.epochMillis = reader.sfixed64();
                        break;
                    case 2:
                        message.nanos = reader.fixed32();
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes a Time message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_core_types.Time
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_core_types.Time} Time
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Time.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies a Time message.
             * @function verify
             * @memberof monitor_core_types.Time
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Time.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.epochMillis != null && message.hasOwnProperty("epochMillis"))
                    if (!$util.isInteger(message.epochMillis) && !(message.epochMillis && $util.isInteger(message.epochMillis.low) && $util.isInteger(message.epochMillis.high)))
                        return "epochMillis: integer|Long expected";
                if (message.nanos != null && message.hasOwnProperty("nanos"))
                    if (!$util.isInteger(message.nanos))
                        return "nanos: integer expected";
                return null;
            };
    
            /**
             * Creates a Time message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_core_types.Time
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_core_types.Time} Time
             */
            Time.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_core_types.Time)
                    return object;
                var message = new $root.monitor_core_types.Time();
                if (object.epochMillis != null)
                    if ($util.Long)
                        (message.epochMillis = $util.Long.fromValue(object.epochMillis)).unsigned = false;
                    else if (typeof object.epochMillis === "string")
                        message.epochMillis = parseInt(object.epochMillis, 10);
                    else if (typeof object.epochMillis === "number")
                        message.epochMillis = object.epochMillis;
                    else if (typeof object.epochMillis === "object")
                        message.epochMillis = new $util.LongBits(object.epochMillis.low >>> 0, object.epochMillis.high >>> 0).toNumber();
                if (object.nanos != null)
                    message.nanos = object.nanos >>> 0;
                return message;
            };
    
            /**
             * Creates a plain object from a Time message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_core_types.Time
             * @static
             * @param {monitor_core_types.Time} message Time
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Time.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (options.defaults) {
                    if ($util.Long) {
                        var long = new $util.Long(0, 0, false);
                        object.epochMillis = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                    } else
                        object.epochMillis = options.longs === String ? "0" : 0;
                    object.nanos = 0;
                }
                if (message.epochMillis != null && message.hasOwnProperty("epochMillis"))
                    if (typeof message.epochMillis === "number")
                        object.epochMillis = options.longs === String ? String(message.epochMillis) : message.epochMillis;
                    else
                        object.epochMillis = options.longs === String ? $util.Long.prototype.toString.call(message.epochMillis) : options.longs === Number ? new $util.LongBits(message.epochMillis.low >>> 0, message.epochMillis.high >>> 0).toNumber() : message.epochMillis;
                if (message.nanos != null && message.hasOwnProperty("nanos"))
                    object.nanos = message.nanos;
                return object;
            };
    
            /**
             * Converts this Time to JSON.
             * @function toJSON
             * @memberof monitor_core_types.Time
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Time.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return Time;
        })();
    
        monitor_core_types.Duration = (function() {
    
            /**
             * Properties of a Duration.
             * @memberof monitor_core_types
             * @interface IDuration
             * @property {number|Long|null} [secs] Duration secs
             * @property {number|null} [nanos] Duration nanos
             */
    
            /**
             * Constructs a new Duration.
             * @memberof monitor_core_types
             * @classdesc Represents a Duration.
             * @implements IDuration
             * @constructor
             * @param {monitor_core_types.IDuration=} [properties] Properties to set
             */
            function Duration(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * Duration secs.
             * @member {number|Long} secs
             * @memberof monitor_core_types.Duration
             * @instance
             */
            Duration.prototype.secs = $util.Long ? $util.Long.fromBits(0,0,false) : 0;
    
            /**
             * Duration nanos.
             * @member {number} nanos
             * @memberof monitor_core_types.Duration
             * @instance
             */
            Duration.prototype.nanos = 0;
    
            /**
             * Creates a new Duration instance using the specified properties.
             * @function create
             * @memberof monitor_core_types.Duration
             * @static
             * @param {monitor_core_types.IDuration=} [properties] Properties to set
             * @returns {monitor_core_types.Duration} Duration instance
             */
            Duration.create = function create(properties) {
                return new Duration(properties);
            };
    
            /**
             * Encodes the specified Duration message. Does not implicitly {@link monitor_core_types.Duration.verify|verify} messages.
             * @function encode
             * @memberof monitor_core_types.Duration
             * @static
             * @param {monitor_core_types.IDuration} message Duration message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Duration.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.secs != null && Object.hasOwnProperty.call(message, "secs"))
                    writer.uint32(/* id 1, wireType 1 =*/9).fixed64(message.secs);
                if (message.nanos != null && Object.hasOwnProperty.call(message, "nanos"))
                    writer.uint32(/* id 2, wireType 5 =*/21).fixed32(message.nanos);
                return writer;
            };
    
            /**
             * Encodes the specified Duration message, length delimited. Does not implicitly {@link monitor_core_types.Duration.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_core_types.Duration
             * @static
             * @param {monitor_core_types.IDuration} message Duration message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Duration.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes a Duration message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_core_types.Duration
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_core_types.Duration} Duration
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Duration.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_core_types.Duration();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 1:
                        message.secs = reader.fixed64();
                        break;
                    case 2:
                        message.nanos = reader.fixed32();
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes a Duration message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_core_types.Duration
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_core_types.Duration} Duration
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Duration.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies a Duration message.
             * @function verify
             * @memberof monitor_core_types.Duration
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Duration.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.secs != null && message.hasOwnProperty("secs"))
                    if (!$util.isInteger(message.secs) && !(message.secs && $util.isInteger(message.secs.low) && $util.isInteger(message.secs.high)))
                        return "secs: integer|Long expected";
                if (message.nanos != null && message.hasOwnProperty("nanos"))
                    if (!$util.isInteger(message.nanos))
                        return "nanos: integer expected";
                return null;
            };
    
            /**
             * Creates a Duration message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_core_types.Duration
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_core_types.Duration} Duration
             */
            Duration.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_core_types.Duration)
                    return object;
                var message = new $root.monitor_core_types.Duration();
                if (object.secs != null)
                    if ($util.Long)
                        (message.secs = $util.Long.fromValue(object.secs)).unsigned = false;
                    else if (typeof object.secs === "string")
                        message.secs = parseInt(object.secs, 10);
                    else if (typeof object.secs === "number")
                        message.secs = object.secs;
                    else if (typeof object.secs === "object")
                        message.secs = new $util.LongBits(object.secs.low >>> 0, object.secs.high >>> 0).toNumber();
                if (object.nanos != null)
                    message.nanos = object.nanos >>> 0;
                return message;
            };
    
            /**
             * Creates a plain object from a Duration message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_core_types.Duration
             * @static
             * @param {monitor_core_types.Duration} message Duration
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Duration.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (options.defaults) {
                    if ($util.Long) {
                        var long = new $util.Long(0, 0, false);
                        object.secs = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                    } else
                        object.secs = options.longs === String ? "0" : 0;
                    object.nanos = 0;
                }
                if (message.secs != null && message.hasOwnProperty("secs"))
                    if (typeof message.secs === "number")
                        object.secs = options.longs === String ? String(message.secs) : message.secs;
                    else
                        object.secs = options.longs === String ? $util.Long.prototype.toString.call(message.secs) : options.longs === Number ? new $util.LongBits(message.secs.low >>> 0, message.secs.high >>> 0).toNumber() : message.secs;
                if (message.nanos != null && message.hasOwnProperty("nanos"))
                    object.nanos = message.nanos;
                return object;
            };
    
            /**
             * Converts this Duration to JSON.
             * @function toJSON
             * @memberof monitor_core_types.Duration
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Duration.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return Duration;
        })();
    
        monitor_core_types.None = (function() {
    
            /**
             * Properties of a None.
             * @memberof monitor_core_types
             * @interface INone
             */
    
            /**
             * Constructs a new None.
             * @memberof monitor_core_types
             * @classdesc Represents a None.
             * @implements INone
             * @constructor
             * @param {monitor_core_types.INone=} [properties] Properties to set
             */
            function None(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * Creates a new None instance using the specified properties.
             * @function create
             * @memberof monitor_core_types.None
             * @static
             * @param {monitor_core_types.INone=} [properties] Properties to set
             * @returns {monitor_core_types.None} None instance
             */
            None.create = function create(properties) {
                return new None(properties);
            };
    
            /**
             * Encodes the specified None message. Does not implicitly {@link monitor_core_types.None.verify|verify} messages.
             * @function encode
             * @memberof monitor_core_types.None
             * @static
             * @param {monitor_core_types.INone} message None message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            None.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                return writer;
            };
    
            /**
             * Encodes the specified None message, length delimited. Does not implicitly {@link monitor_core_types.None.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_core_types.None
             * @static
             * @param {monitor_core_types.INone} message None message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            None.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes a None message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_core_types.None
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_core_types.None} None
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            None.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_core_types.None();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes a None message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_core_types.None
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_core_types.None} None
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            None.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies a None message.
             * @function verify
             * @memberof monitor_core_types.None
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            None.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                return null;
            };
    
            /**
             * Creates a None message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_core_types.None
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_core_types.None} None
             */
            None.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_core_types.None)
                    return object;
                return new $root.monitor_core_types.None();
            };
    
            /**
             * Creates a plain object from a None message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_core_types.None
             * @static
             * @param {monitor_core_types.None} message None
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            None.toObject = function toObject() {
                return {};
            };
    
            /**
             * Converts this None to JSON.
             * @function toJSON
             * @memberof monitor_core_types.None
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            None.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return None;
        })();
    
        return monitor_core_types;
    })();

    return $root;
})(protobuf);
