package flash.geom {
    import __ruffle__.stub_getter;
    import __ruffle__.stub_method;
    import __ruffle__.stub_setter;
    import flash.display.DisplayObject;
    import flash.geom.Matrix3D;
    import flash.geom.Point;

    public class PerspectiveProjection {
        [Ruffle(NativeAccessible)]
        private var displayObject: DisplayObject = null;

        [Ruffle(NativeAccessible)]
        private var _focalLength: Number;

        [Ruffle(NativeAccessible)]
        private var center: Point = new Point(250, 250);

        public function PerspectiveProjection() {
            this.fieldOfView = 55.0;
        }

        public native function get fieldOfView():Number;

        public native function set fieldOfView(value:Number);

        public function get focalLength():Number {
            return this._focalLength;
        }

        public function set focalLength(value:Number) {
            if (value <= 0.0) {
                throw new ArgumentError("Error #2186: Invalid focalLength " + value + ".", 2186);
            }
            this._focalLength = value;
        }

        public function get projectionCenter():Point {
            stub_getter("flash.geom.PerspectiveProjection", "projectionCenter");
            return this.center;
        }
        public function set projectionCenter(value:Point) {
            // TODO: This setter should update the associated displayObject when there is.
            stub_setter("flash.geom.PerspectiveProjection", "projectionCenter");
            this.center = value;
        }

        public function toMatrix3D():Matrix3D {
            var fl: Number = this.focalLength;
            return new Matrix3D(new <Number>[
              fl, 0, 0, 0,
              0, fl, 0, 0,
              0, 0, 1, 1,
              0, 0, 0, 0
            ]);
        }
    }
}
