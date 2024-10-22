namespace Zengine {
    class Zobject
    {
    private:
        /* data */
    public:
        int instance_id;
        Zobject(/* args */);
        ~Zobject();
        void _init() {

        };
    };
    
    Zobject::Zobject(/* args */)
    {
        instance_id = $insert random int here
    }
    
    Zobject::~Zobject()
    {
    }
    
}